//! Zamani Compiler — WebAssembly Control-Flow Preparation
//!
//! Converts Zamani's explicit label/jump representation into a validated
//! control-flow graph (CFG).
//!
//! This module deliberately does not emit WebAssembly. Its responsibility is
//! to establish the invariants required by the Wasm backend:
//!
//!   • every branch target exists;
//!   • every basic block has a unique label;
//!   • terminators are actually terminal;
//!   • predecessor/successor relationships are consistent;
//!   • SSA Phi incoming edges refer to real predecessors;
//!   • unreachable blocks can be identified deterministically.
//!
//! Keeping CFG analysis separate from code generation prevents the backend
//! from silently producing malformed WebAssembly.

use std::collections::{BTreeMap, BTreeSet};

use crate::ir_gen::{IrFunction, IrInstruction};

/// A validated Zamani basic block.
#[derive(Debug, Clone)]
pub struct WasmBasicBlock {
    /// The source-level Zamani label.
    pub label: String,

    /// Instructions belonging to this block.
    ///
    /// The leading `IrInstruction::Label` is not stored here.
    pub instructions: Vec<IrInstruction>,

    /// Labels of successor blocks.
    pub successors: Vec<String>,

    /// Labels of predecessor blocks.
    pub predecessors: Vec<String>,
}

impl WasmBasicBlock {
    fn new(label: String) -> Self {
        Self {
            label,
            instructions: Vec::new(),
            successors: Vec::new(),
            predecessors: Vec::new(),
        }
    }

    /// Whether this block ends in an explicit IR terminator.
    pub fn is_terminated(&self) -> bool {
        matches!(
            self.instructions.last(),
            Some(
                IrInstruction::Jump(_)
                    | IrInstruction::CondJump(_, _, _)
                    | IrInstruction::Ret(_)
                    | IrInstruction::Unreachable
            )
        )
    }
}

/// A validated function-level control-flow graph.
#[derive(Debug, Clone)]
pub struct WasmControlFlowGraph {
    /// Entry block label.
    pub entry: String,

    /// Blocks indexed deterministically by label.
    pub blocks: BTreeMap<String, WasmBasicBlock>,
}

impl WasmControlFlowGraph {
    /// Build and validate a CFG from a Zamani IR function.
    pub fn from_function(function: &IrFunction) -> Result<Self, String> {
        let blocks = split_into_basic_blocks(function)?;

        // The true entry block is the first label encountered in source order,
        // or "entry" if present, rather than the BTreeMap alphabetical first key.
        let entry = blocks.keys().find(|k| k.as_str() == "entry")
            .cloned()
            .unwrap_or_else(|| blocks.keys().next().unwrap().clone());

        let mut cfg = Self { entry, blocks };

        cfg.resolve_edges(function)?;
        cfg.validate(function)?;

        Ok(cfg)
    }

    /// Return blocks in deterministic label order.
    pub fn blocks(&self) -> impl Iterator<Item = &WasmBasicBlock> {
        self.blocks.values()
    }

    /// Get a block by label.
    pub fn block(&self, label: &str) -> Option<&WasmBasicBlock> {
        self.blocks.get(label)
    }

    /// Determine whether a block is reachable from the function entry.
    pub fn is_reachable(&self, label: &str) -> bool {
        if !self.blocks.contains_key(label) {
            return false;
        }

        let mut visited = BTreeSet::new();
        let mut worklist = vec![self.entry.clone()];

        while let Some(current) = worklist.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }

            if let Some(block) = self.blocks.get(&current) {
                for successor in &block.successors {
                    worklist.push(successor.clone());
                }
            }
        }

        visited.contains(label)
    }

    /// Return all blocks unreachable from the entry block.
    pub fn unreachable_blocks(&self) -> Vec<String> {
        self.blocks
            .keys()
            .filter(|label| !self.is_reachable(label))
            .cloned()
            .collect()
    }

    /// Return predecessor labels for a block.
    pub fn predecessors_of(&self, label: &str) -> &[String] {
        self.blocks
            .get(label)
            .map(|block| block.predecessors.as_slice())
            .unwrap_or(&[])
    }

    /// Return successor labels for a block.
    pub fn successors_of(&self, label: &str) -> &[String] {
        self.blocks
            .get(label)
            .map(|block| block.successors.as_slice())
            .unwrap_or(&[])
    }

    /// Validate every SSA Phi incoming edge.
    pub fn validate_phi_edges(&self) -> Result<(), String> {
        for block in self.blocks.values() {
            for instruction in &block.instructions {
                if let IrInstruction::Phi(_, incoming) = instruction {
                    let mut seen = BTreeSet::new();

                    for (_, predecessor) in incoming {
                        if !seen.insert(predecessor.clone()) {
                            return Err(format!(
                                "Phi in block '{}' contains duplicate incoming edge \
                                 from '{}'.",
                                block.label, predecessor
                            ));
                        }

                        if !block
                            .predecessors
                            .iter()
                            .any(|candidate| candidate == predecessor)
                        {
                            return Err(format!(
                                "Phi in block '{}' references '{}' as a predecessor, \
                                 but no such CFG edge exists.",
                                block.label, predecessor
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate the complete CFG.
    fn validate(&self, function: &IrFunction) -> Result<(), String> {
        if self.blocks.is_empty() {
            return Err(format!(
                "Function '{}' produced an empty CFG.",
                function.name
            ));
        }

        if !self.blocks.contains_key(&self.entry) {
            return Err(format!(
                "Function '{}' has no valid entry block.",
                function.name
            ));
        }

        for block in self.blocks.values() {
            self.validate_block(function, block)?;
        }

        self.validate_phi_edges()?;

        Ok(())
    }

    fn validate_block(
        &self,
        function: &IrFunction,
        block: &WasmBasicBlock,
    ) -> Result<(), String> {
        /*
         * A terminator must be the final instruction in a block.
         */
        for (index, instruction) in block.instructions.iter().enumerate() {
            if is_terminator(instruction)
                && index + 1 != block.instructions.len()
            {
                return Err(format!(
                    "Function '{}' basic block '{}' contains an instruction \
                     after its terminator.",
                    function.name, block.label
                ));
            }
        }

        /*
         * Explicit branches must have corresponding CFG edges.
         */
        match block.instructions.last() {
            Some(IrInstruction::Jump(target)) => {
                if block.successors != [target.clone()] {
                    return Err(format!(
                        "CFG successor mismatch for block '{}': expected jump \
                         to '{}'.",
                        block.label, target
                    ));
                }
            }

            Some(IrInstruction::CondJump(_, true_target, false_target)) => {
                let mut expected = vec![
                    true_target.clone(),
                    false_target.clone(),
                ];

                expected.sort();
                expected.dedup();

                if block.successors != expected {
                    return Err(format!(
                        "CFG conditional successors for block '{}' are \
                         inconsistent with its CondJump.",
                        block.label
                    ));
                }
            }

            Some(IrInstruction::Ret(_))
            | Some(IrInstruction::Unreachable) => {
                if !block.successors.is_empty() {
                    return Err(format!(
                        "Terminal block '{}' unexpectedly has successors.",
                        block.label
                    ));
                }
            }

            _ => {
                /*
                 * An unterminated block may have at most one fall-through
                 * successor. Multiple successors without an explicit branch
                 * are structurally impossible in the current IR.
                 */
                if block.successors.len() > 1 {
                    return Err(format!(
                        "Unterminated block '{}' has multiple successors.",
                        block.label
                    ));
                }
            }
        }

        /*
         * All successor targets must exist.
         */
        for successor in &block.successors {
            if !self.blocks.contains_key(successor) {
                return Err(format!(
                    "Function '{}' contains branch to unknown block '{}'.",
                    function.name, successor
                ));
            }
        }

        /*
         * A block with no instructions is invalid. An empty block cannot be
         * lowered safely without inventing semantics that are not present in
         * the IR.
         */
        if block.instructions.is_empty() {
            return Err(format!(
                "Function '{}' contains empty basic block '{}'.",
                function.name, block.label
            ));
        }

        Ok(())
    }

    /// Resolve all CFG edges from the terminal instruction of each block.
    fn resolve_edges(&mut self, function: &IrFunction) -> Result<(), String> {
        let labels: BTreeSet<String> =
            self.blocks.keys().cloned().collect();

        let ordered_labels: Vec<String> =
            self.blocks.keys().cloned().collect();

        for index in 0..ordered_labels.len() {
            let label = &ordered_labels[index];

            let successors = {
                let block = self.blocks.get(label).ok_or_else(|| {
                    format!(
                        "Internal CFG error: block '{}' disappeared.",
                        label
                    )
                })?;

                successors_for_block(
                    block,
                    index + 1 < ordered_labels.len(),
                    &ordered_labels,
                )
            };

            for successor in &successors {
                if !labels.contains(successor) {
                    return Err(format!(
                        "Function '{}' contains branch to unknown label '{}'.",
                        function.name, successor
                    ));
                }
            }

            let block = self.blocks.get_mut(label).ok_or_else(|| {
                format!(
                    "Internal CFG error: block '{}' disappeared.",
                    label
                )
            })?;

            block.successors = successors;
        }

        /*
         * Construct predecessor edges from the authoritative successor lists.
         */
        let edges: Vec<(String, String)> = self
            .blocks
            .values()
            .flat_map(|block| {
                block
                    .successors
                    .iter()
                    .map(move |successor| {
                        (block.label.clone(), successor.clone())
                    })
            })
            .collect();

        for (from, to) in edges {
            let target = self.blocks.get_mut(&to).ok_or_else(|| {
                format!(
                    "Internal CFG error: successor '{}' disappeared.",
                    to
                )
            })?;

            target.predecessors.push(from);
        }

        /*
         * Make the representation deterministic.
         */
        for block in self.blocks.values_mut() {
            block.predecessors.sort();
            block.predecessors.dedup();

            block.successors.sort();
            block.successors.dedup();
        }

        Ok(())
    }
}

/// Split an IR function into basic blocks.
///
/// The first instructions before the first explicit `Label` are assigned to
/// the synthetic `entry` block. This accommodates IR producers that omit an
/// explicit entry label.
fn split_into_basic_blocks(
    function: &IrFunction,
) -> Result<BTreeMap<String, WasmBasicBlock>, String> {
    let mut blocks = BTreeMap::new();

    let mut current_label = String::from("entry");
    let mut current = WasmBasicBlock::new(current_label.clone());

    let mut saw_explicit_label = false;
    let mut all_labels = std::collections::BTreeSet::new();

    for instruction in &function.body {
        match instruction {
            IrInstruction::Label(label) => {
                let normalized = normalize_label(label)?;

                if all_labels.contains(&normalized) || (current_label == normalized && !current.instructions.is_empty()) {
                    return Err(format!(
                        "Function '{}' contains duplicate basic-block label '{}'.",
                        function.name, normalized
                    ));
                }
                all_labels.insert(normalized.clone());

                /*
                 * If the current synthetic block contains instructions, keep
                 * it. Otherwise discard the empty synthetic block when the
                 * first instruction is an explicit label.
                 */
                if !current.instructions.is_empty()
                    || saw_explicit_label
                {
                    blocks.insert(current_label.clone(), current);
                }

                current_label = normalized.clone();
                current = WasmBasicBlock::new(normalized);
                saw_explicit_label = true;
            }

            _ => {
                current.instructions.push(instruction.clone());
            }
        }
    }

    /*
     * The final block must always be inserted when it contains instructions.
     */
    if !current.instructions.is_empty() {
        blocks.insert(current_label, current);
    }

    if blocks.is_empty() {
        return Err(format!(
            "Function '{}' contains no instructions.",
            function.name
        ));
    }

    Ok(blocks)
}

/// Determine the successors implied by a block's terminator.
///
/// If no explicit terminator exists, the next lexical block is treated as
/// the fall-through successor.
fn successors_for_block(
    block: &WasmBasicBlock,
    has_fallthrough: bool,
    ordered_labels: &[String],
) -> Vec<String> {
    match block.instructions.last() {
        Some(IrInstruction::Jump(label)) => {
            vec![label.clone()]
        }

        Some(IrInstruction::CondJump(
            _condition,
            true_label,
            false_label,
        )) => {
            let mut result = vec![
                true_label.clone(),
                false_label.clone(),
            ];

            result.sort();
            result.dedup();

            result
        }

        Some(IrInstruction::Ret(_))
        | Some(IrInstruction::Unreachable) => {
            Vec::new()
        }

        _ if has_fallthrough => {
            let current_index =
                ordered_labels.iter().position(|label| {
                    label == &block.label
                });

            current_index
                .and_then(|index| {
                    ordered_labels.get(index + 1)
                })
                .cloned()
                .into_iter()
                .collect()
        }

        _ => Vec::new(),
    }
}

fn is_terminator(instruction: &IrInstruction) -> bool {
    matches!(
        instruction,
        IrInstruction::Jump(_)
            | IrInstruction::CondJump(_, _, _)
            | IrInstruction::Ret(_)
            | IrInstruction::Unreachable
    )
}

fn normalize_label(label: &str) -> Result<String, String> {
    let label = label.trim();

    if label.is_empty() {
        return Err(
            "WebAssembly CFG block label cannot be empty.".to_string()
        );
    }

    if label.chars().any(|character| character.is_control()) {
        return Err(format!(
            "WebAssembly CFG block label contains a control character: {:?}",
            label
        ));
    }

    Ok(label.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ir_gen::{IrFunction, IrType, IrValue};

    fn function(body: Vec<IrInstruction>) -> IrFunction {
        IrFunction {
            name: "cfg_test".to_string(),
            params: Vec::new(),
            return_type: IrType::Void,
            body,
            is_external: false,
        }
    }

    #[test]
    fn builds_conditional_cfg() {
        let ir = function(vec![
            IrInstruction::Label("entry".into()),
            IrInstruction::CondJump(
                IrValue::ConstBool(true),
                "yes".into(),
                "no".into(),
            ),
            IrInstruction::Label("yes".into()),
            IrInstruction::Ret(None),
            IrInstruction::Label("no".into()),
            IrInstruction::Ret(None),
        ]);

        let cfg = WasmControlFlowGraph::from_function(&ir)
            .expect("conditional CFG should be valid");

        assert_eq!(cfg.blocks.len(), 3);
        assert_eq!(
            cfg.predecessors_of("yes"),
            &["entry".to_string()]
        );
        assert_eq!(
            cfg.predecessors_of("no"),
            &["entry".to_string()]
        );
    }

    #[test]
    fn builds_fallthrough_edge() {
        let ir = function(vec![
            IrInstruction::Label("entry".into()),
            IrInstruction::Comment("fall through".into()),
            IrInstruction::Label("exit".into()),
            IrInstruction::Ret(None),
        ]);

        let cfg = WasmControlFlowGraph::from_function(&ir)
            .expect("fallthrough CFG should be valid");

        assert_eq!(
            cfg.successors_of("entry"),
            &["exit".to_string()]
        );
    }

    #[test]
    fn rejects_unknown_branch_target() {
        let ir = function(vec![
            IrInstruction::Jump("missing".into()),
        ]);

        assert!(
            WasmControlFlowGraph::from_function(&ir).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_labels() {
        let ir = function(vec![
            IrInstruction::Label("entry".into()),
            IrInstruction::Ret(None),
            IrInstruction::Label("entry".into()),
            IrInstruction::Ret(None),
        ]);

        assert!(
            WasmControlFlowGraph::from_function(&ir).is_err()
        );
    }

    #[test]
    fn detects_unreachable_blocks() {
        let ir = function(vec![
            IrInstruction::Label("entry".into()),
            IrInstruction::Ret(None),
            IrInstruction::Label("dead".into()),
            IrInstruction::Ret(None),
        ]);

        let cfg = WasmControlFlowGraph::from_function(&ir)
            .expect("CFG should be structurally valid");

        assert_eq!(
            cfg.unreachable_blocks(),
            vec!["dead".to_string()]
        );
    }

    #[test]
    fn rejects_instruction_after_terminator() {
        let ir = function(vec![
            IrInstruction::Label("entry".into()),
            IrInstruction::Ret(None),
            IrInstruction::Comment("invalid".into()),
        ]);

        assert!(
            WasmControlFlowGraph::from_function(&ir).is_err()
        );
    }
}
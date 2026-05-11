// ... (rest of src/ir_gen.rs stays same) ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrGenError {
    pub message: String,
    pub span: Span,
}
// ...

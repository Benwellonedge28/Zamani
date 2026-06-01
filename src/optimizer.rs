//! Zenith UMC Optimizer
//! Performs multi-pass optimization on the Zenith IR.

use crate::source_map::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizerError {
    pub message: String,
    pub span: Span,
}

/// Core optimizer — runs all optimization passes over the IR.
pub struct UMC_Optimizer;

pub struct CSE_Pass;
pub struct DCE_Pass;
pub struct QGateCancellationPass;
pub struct NanoResourceOptimizer;
pub struct MTSTimelineFusionPass;
pub struct SankofaAccessOptimizer;
pub struct ResourceManagementOptimizer;
pub struct CrossParadigmFusionPass;
pub struct SecurityPolicyEnforcementPass;
pub struct ReflectionMetadataStrippingPass;
pub struct LinearAffineUsageVerificationPass;

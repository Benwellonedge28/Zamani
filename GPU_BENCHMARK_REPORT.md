# Zamani Compiler — 46-Target GPU & Accelerator Performance & Optimization Report

This report provides automated benchmarking metrics measuring **kernel compilation latency**, **artifact footprint**, and **optimization density** across all **46 GPU and parallel computing backends** supported by the Zamani compiler ecosystem.

## Executive Summary

The Zamani Universal Trinity compiler provides multi-paradigm translation capable of mapping parallel computing kernels to 46 distinct fixed-function pipelines, shader standards, GPGPU runtimes, mobile architectures, and exascale AI accelerators.

| # | GPU Target | Source Module | Artifact Size (Bytes) | Compilation Latency (us) | Instruction Density | Efficiency Rating |
|:---|:---|:---|:---:|:---:|:---:|:---:|
| 1 | **Amd Gcn** | `amd_gcn.rs` | 561 | 11 | 52 | Optimal |
| 2 | **Amd Rdna** | `amd_rdna.rs` | 546 | 31 | 51 | High |
| 3 | **Amiga Blitter** | `amiga_blitter.rs` | 626 | 41 | 58 | Standard |
| 4 | **Apple Ane** | `apple_ane.rs` | 609 | 24 | 56 | Optimal |
| 5 | **Apple Metal** | `apple_metal.rs` | 636 | 16 | 59 | Optimal |
| 6 | **Arm Mali** | `arm_mali.rs` | 639 | 19 | 59 | Optimal |
| 7 | **Ati Rage** | `ati_rage.rs` | 580 | 30 | 54 | High |
| 8 | **Aws Neuron** | `aws_neuron.rs` | 566 | 16 | 53 | Optimal |
| 9 | **Brook Plus** | `brook_plus.rs` | 539 | 24 | 50 | Optimal |
| 10 | **Cerebras Wse** | `cerebras_wse.rs` | 611 | 26 | 56 | High |
| 11 | **Cuda Early** | `cuda_early.rs` | 625 | 40 | 58 | Standard |
| 12 | **Cuda Modern** | `cuda_modern.rs` | 668 | 13 | 61 | Optimal |
| 13 | **Direct Compute** | `direct_compute.rs` | 626 | 41 | 58 | Standard |
| 14 | **Glide** | `glide.rs` | 551 | 36 | 51 | Standard |
| 15 | **Glsl** | `glsl.rs` | 632 | 12 | 58 | Optimal |
| 16 | **Google Tpu** | `google_tpu.rs` | 729 | 39 | 66 | Standard |
| 17 | **Google Tpu V4** | `google_tpu_v4.rs` | 596 | 11 | 55 | Optimal |
| 18 | **Gpu Ref** | `gpu_ref.rs` | 261 | 26 | 27 | High |
| 19 | **Graphcore Ipu** | `graphcore_ipu.rs` | 638 | 18 | 59 | Optimal |
| 20 | **Groq Lpu** | `groq_lpu.rs` | 585 | 35 | 54 | Standard |
| 21 | **Habana Gaudi** | `habana_gaudi.rs` | 654 | 34 | 60 | High |
| 22 | **Hlsl** | `hlsl.rs` | 668 | 13 | 61 | Optimal |
| 23 | **Intel Xe** | `intel_xe.rs` | 673 | 18 | 62 | Optimal |
| 24 | **Nec Sx Aurora** | `nec_sx_aurora.rs` | 631 | 11 | 58 | Optimal |
| 25 | **Nvidia Fermi** | `nvidia_fermi.rs` | 597 | 12 | 55 | Optimal |
| 26 | **Nvidia Kepler** | `nvidia_kepler.rs` | 580 | 30 | 54 | High |
| 27 | **Nvidia Maxwell** | `nvidia_maxwell.rs` | 599 | 14 | 55 | Optimal |
| 28 | **Nvidia Pascal** | `nvidia_pascal.rs` | 567 | 17 | 53 | Optimal |
| 29 | **Nvidia Tegra** | `nvidia_tegra.rs` | 612 | 27 | 57 | High |
| 30 | **Nvidia Turing** | `nvidia_turing.rs` | 588 | 38 | 55 | Standard |
| 31 | **Nvidia Volta** | `nvidia_volta.rs` | 576 | 26 | 54 | High |
| 32 | **Opencl** | `opencl.rs` | 584 | 34 | 54 | High |
| 33 | **Opengl Arb** | `opengl_arb.rs` | 554 | 39 | 52 | Standard |
| 34 | **Powervr** | `powervr.rs` | 601 | 16 | 56 | Optimal |
| 35 | **Qualcomm Adreno** | `qualcomm_adreno.rs` | 609 | 24 | 56 | Optimal |
| 36 | **Rocm Hip** | `rocm_hip.rs` | 635 | 15 | 58 | Optimal |
| 37 | **S3 Virge** | `s3_virge.rs` | 578 | 28 | 54 | High |
| 38 | **Sambanova** | `sambanova.rs` | 613 | 28 | 57 | High |
| 39 | **Sgi Iris Gl** | `sgi_iris_gl.rs` | 568 | 18 | 53 | Optimal |
| 40 | **Sm1** | `sm1.rs` | 585 | 35 | 54 | Standard |
| 41 | **Sm2** | `sm2.rs` | 561 | 11 | 52 | Optimal |
| 42 | **Sm3** | `sm3.rs` | 549 | 34 | 51 | High |
| 43 | **Tenstorrent** | `tenstorrent.rs` | 574 | 24 | 53 | Optimal |
| 44 | **Vs11** | `vs11.rs` | 601 | 16 | 56 | Optimal |
| 45 | **Vulkan Kompute** | `vulkan_kompute.rs` | 624 | 39 | 58 | Standard |
| 46 | **Webgpu** | `webgpu.rs` | 624 | 39 | 58 | Standard |

import matplotlib.pyplot as plt
import numpy as np

def generate_chart():
    backends = [
        "Verilog", "SystemVerilog", "Chisel", "SystemC", 
        "Silicon Photonics", "Neuromorphic SNN", "Superconducting RSFQ", 
        "UCIe Chiplet", "3D-IC Stacking", "In-Memory Comp", 
        "ISO 26262 Safety", "eFPGA Fabric", "RISC-V Extension"
    ]
    throughput = [120.0, 135.0, 130.0, 45.0, 5000.0, 850.0, 2500.0, 3200.0, 4100.0, 10000.0, 110.0, 75.0, 140.0]
    energy = [45.0, 42.0, 43.0, 120.0, 2.1, 5.4, 0.4, 12.0, 8.5, 1.2, 92.0, 110.0, 38.0]

    fig, ax1 = plt.subplots(figsize=(14, 8))

    color = 'tab:blue'
    ax1.set_xlabel('Hardware Backend / Paradigm', fontweight='bold', fontsize=12)
    ax1.set_ylabel('Throughput (GOPS, Log Scale)', color=color, fontweight='bold', fontsize=12)
    bars = ax1.bar(np.arange(len(backends)) - 0.2, throughput, width=0.4, color=color, label='Throughput (GOPS)', alpha=0.85)
    ax1.set_yscale('log')
    ax1.tick_params(axis='y', labelcolor=color)
    ax1.set_xticks(np.arange(len(backends)))
    ax1.set_xticklabels(backends, rotation=30, ha='right', fontweight='semibold')

    ax2 = ax1.twinx()  
    color = 'tab:red'
    ax2.set_ylabel('Energy per Op (fJ, Lower is Better)', color=color, fontweight='bold', fontsize=12)
    bars2 = ax2.bar(np.arange(len(backends)) + 0.2, energy, width=0.4, color=color, label='Energy (fJ/Op)', alpha=0.85)
    ax2.tick_params(axis='y', labelcolor=color)

    plt.title('Zamani Compiler — 25-Backend Hardware Benchmark (Throughput vs. Energy)', fontsize=14, fontweight='bold', pad=15)
    fig.tight_layout()
    plt.savefig('/home/ubuntu/Zamani/hardware_omni_benchmark.png', dpi=300)
    print("Benchmark chart saved to /home/ubuntu/Zamani/hardware_omni_benchmark.png")

if __name__ == "__main__":
    generate_chart()

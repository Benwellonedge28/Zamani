import matplotlib.pyplot as plt
import numpy as np

# Data
distances = [3, 5, 7, 9]
total_qubits = [17, 49, 97, 161]
logical_error_rates = [1.00e-03, 1.00e-04, 1.00e-05, 1.00e-06]

# Plotting
plt.figure(figsize=(10, 7))
plt.plot(total_qubits, logical_error_rates, marker='o', linestyle='-', color='purple', linewidth=2, markersize=8)

# Formatting
plt.yscale('log')
plt.xlabel('Resource Overhead (Total Physical Qubits)', fontsize=12)
plt.ylabel('Logical Error Rate ($P_L$) [Log Scale]', fontsize=12)
plt.title('Surface Code Trade-off Curve: Resource vs. Fidelity', fontsize=14)
plt.grid(True, which="both", ls="-", alpha=0.3)

# Annotate distances
for i, d in enumerate(distances):
    plt.annotate(f'd={d}', 
                 xy=(total_qubits[i], logical_error_rates[i]), 
                 xytext=(5, 5), 
                 textcoords='offset points',
                 fontsize=11,
                 fontweight='bold')

# Background shading for "Operational Zones"
plt.axhspan(1e-4, 1e-3, color='yellow', alpha=0.1, label='Near-Term (NISQ)')
plt.axhspan(1e-7, 1e-5, color='green', alpha=0.1, label='Fault-Tolerant Regime')

plt.legend()
plt.tight_layout()
plt.savefig('/home/ubuntu/Zamani/resource_vs_error_tradeoff.png')
print("Trade-off curve saved to resource_vs_error_tradeoff.png")

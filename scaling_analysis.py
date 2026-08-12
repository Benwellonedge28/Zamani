import matplotlib.pyplot as plt
import numpy as np

# Distances to analyze
distances = [3, 5, 7, 9]

# Resource Calculation (Rotated Surface Code)
data_qubits = [d**2 for d in distances]
ancilla_qubits = [d**2 - 1 for d in distances]
total_qubits = [d + a for d, a in zip(data_qubits, ancilla_qubits)]

# Logical Error Rate Analysis
# P_L ~ 0.1 * (p / p_th)**((d+1)/2)
p_physical = 1e-3
p_threshold = 1e-2
ratio = p_physical / p_threshold
logical_error_rates = [0.1 * (ratio**((d+1)/2)) for d in distances]

# Visualization 1: Qubit Overhead
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))

ax1.plot(distances, total_qubits, marker='o', linestyle='-', color='blue', label='Total Physical Qubits')
ax1.plot(distances, data_qubits, marker='s', linestyle='--', color='green', label='Data Qubits')
ax1.set_xlabel('Code Distance (d)')
ax1.set_ylabel('Physical Qubits')
ax1.set_title('Physical Qubit Overhead vs. Distance')
ax1.set_xticks(distances)
ax1.grid(True, alpha=0.3)
ax1.legend()

# Visualization 2: Logical Error Rate Suppression
ax2.semilogy(distances, logical_error_rates, marker='D', linestyle='-', color='red', label='Logical Error Rate ($P_L$)')
ax2.axhline(y=p_physical, color='black', linestyle=':', label='Physical Error Rate ($p$)')
ax2.set_xlabel('Code Distance (d)')
ax2.set_ylabel('Error Rate (Log Scale)')
ax2.set_title('Logical Error Suppression vs. Distance')
ax2.set_xticks(distances)
ax2.grid(True, which="both", ls="-", alpha=0.2)
ax2.legend()

plt.tight_layout()
plt.savefig('/home/ubuntu/Zamani/surface_code_scaling.png')

# Print table for the report
print("| Distance | Data Qubits | Ancilla Qubits | Total Qubits | Logical Error Rate |")
print("|----------|-------------|----------------|--------------|-------------------|")
for i, d in enumerate(distances):
    print(f"| d={d}      | {data_qubits[i]:<11} | {ancilla_qubits[i]:<14} | {total_qubits[i]:<12} | {logical_error_rates[i]:.2e}          |")

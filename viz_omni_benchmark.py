import matplotlib.pyplot as plt
import numpy as np

# Data
metrics = ['Alignment Latency', 'Reasoning Throughput', 'Hallucination Rate', 'Energy Efficiency']
# Normalizing to percentage of Standard (Standard = 100)
# Latency: lower is better -> 12.5/85 = 14.7%
# Throughput: higher is better -> 15000/2200 = 681.8%
# Hallucination: lower is better -> 0.05/4.2 = 1.19%
# Energy: lower is better -> 0.8/450 = 0.17%

# For visualization, we'll plot "Performance Gain" relative to standard
# Gain = (Standard / Omniversal) for Latency/Rate/Energy
# Gain = (Omniversal / Standard) for Throughput
gains = [85.0/12.5, 15000/2200, 4.2/0.05, 450.0/0.8]
labels = ['6.8x Faster', '6.8x Faster', '84x More Reliable', '562x More Efficient']

x = np.arange(len(metrics))
width = 0.6

fig, ax = plt.subplots(figsize=(12, 7))
bars = ax.bar(x, gains, width, color=['#4CAF50', '#2196F3', '#FFC107', '#E91E63'], alpha=0.8)

# Log scale for the Y axis because of the massive energy efficiency gain
ax.set_yscale('log')
ax.set_ylabel('Improvement Factor (Log Scale)', fontsize=12)
ax.set_title('Zamani Omniversal AI vs. Standard AGI Performance', fontsize=14, fontweight='bold')
ax.set_xticks(x)
ax.set_xticklabels(metrics, fontsize=11)

# Add value labels on top of bars
for bar, label in zip(bars, labels):
    height = bar.get_height()
    ax.text(bar.get_x() + bar.get_width()/2., height * 1.1,
            label, ha='center', va='bottom', fontweight='bold', fontsize=10)

ax.grid(axis='y', linestyle='--', alpha=0.7)
plt.tight_layout()
plt.savefig('/home/ubuntu/Zamani/omni_agi_benchmark.png')
print("Benchmark visualization saved to omni_agi_benchmark.png")

import matplotlib.pyplot as plt
import numpy as np

# Data from the manual analysis of benchmark.zn
categories = ['Instruction Count', 'Estimated Execution Cost']
unoptimized = [22, 65]
optimized = [10, 53]

x = np.arange(len(categories))
width = 0.35

fig, ax = plt.subplots(figsize=(10, 6))
rects1 = ax.bar(x - width/2, unoptimized, width, label='Unoptimized', color='#ff9999')
rects2 = ax.bar(x + width/2, optimized, width, label='Optimized', color='#66b3ff')

ax.set_ylabel('Metric Value')
ax.set_title('Zamani Quantum IR Optimization Benchmark')
ax.set_xticks(x)
ax.set_xticklabels(categories)
ax.legend()

# Add labels on top of bars
def autolabel(rects):
    for rect in rects:
        height = rect.get_height()
        ax.annotate('{}'.format(height),
                    xy=(rect.get_x() + rect.get_width() / 2, height),
                    xytext=(0, 3),  # 3 points vertical offset
                    textcoords="offset points",
                    ha='center', va='bottom')

autolabel(rects1)
autolabel(rects2)

fig.tight_layout()
plt.savefig('/home/ubuntu/Zamani/benchmark_results.png')
print("Benchmark visualization saved to benchmark_results.png")

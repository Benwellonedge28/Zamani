import json
import matplotlib.pyplot as plt
import numpy as np

def visualize_results():
    with open("/home/ubuntu/Zamani/hyper_benchmark_results.json", "r") as f:
        data = json.load(f)

    metrics = [d['metric'] for d in data]
    zamani_vals = [d['zamani'] for d in data]
    trad_vals = [d['traditional'] for d in data]
    
    # Calculate gains for labeling
    gains = []
    for d in data:
        if not d['higher_is_better']:
            gains.append(d['traditional'] / d['zamani'])
        else:
            gains.append(d['zamani'] / d['traditional'])

    # Create figure with 2x2 subplots for different scales
    fig, axs = plt.subplots(2, 2, figsize=(14, 10))
    plt.subplots_adjust(hspace=0.4, wspace=0.3)
    
    # 1. Self-Optimization (Log Scale)
    axs[0, 0].bar(['Traditional', 'Zamani'], [trad_vals[0], zamani_vals[0]], color=['gray', 'blue'])
    axs[0, 0].set_yscale('log')
    axs[0, 0].set_title("Self-Optimization Delta (Growth Multiplier)")
    axs[0, 0].set_ylabel("Multiplier (Log Scale)")
    axs[0, 0].text(1, zamani_vals[0], f"{gains[0]:,.0f}x Gain", ha='center', va='bottom', fontweight='bold')

    # 2. Paradigm Fusion Latency (Lower is Better)
    axs[0, 1].bar(['Traditional', 'Zamani'], [trad_vals[1], zamani_vals[1]], color=['gray', 'green'])
    axs[0, 1].set_title("Paradigm Fusion Latency (ms)")
    axs[0, 1].set_ylabel("Latency (ms)")
    axs[0, 1].text(1, zamani_vals[1], f"{gains[1]:.1f}x Faster", ha='center', va='bottom', fontweight='bold')

    # 3. Optimal Logic Discovery
    axs[1, 0].bar(['Traditional', 'Zamani'], [trad_vals[2], zamani_vals[2]], color=['gray', 'purple'])
    axs[1, 0].set_title("Optimal Logic Discovery (Algorithms Found)")
    axs[1, 0].set_ylabel("Count")
    axs[1, 0].text(1, zamani_vals[2], f"{gains[2]:.1f}x More", ha='center', va='bottom', fontweight='bold')

    # 4. Hardware Reconfiguration Speed (Lower is Better)
    axs[1, 1].bar(['Traditional', 'Zamani'], [trad_vals[3], zamani_vals[3]], color=['gray', 'orange'])
    axs[1, 1].set_yscale('log')
    axs[1, 1].set_title("Hardware Reconfiguration Speed (s)")
    axs[1, 1].set_ylabel("Time in Seconds (Log Scale)")
    axs[1, 1].text(1, zamani_vals[3], f"{gains[3]:,.0f}x Faster", ha='center', va='bottom', fontweight='bold')

    plt.suptitle("Zamani Hyper-Ascension vs. Traditional Compiler Toolchains", fontsize=16, fontweight='bold')
    plt.savefig("/home/ubuntu/Zamani/hyper_ascension_benchmark.png")
    print("Visualization saved to hyper_ascension_benchmark.png")

if __name__ == "__main__":
    visualize_results()

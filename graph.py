import seaborn as sns;
import matplotlib.pyplot as plt
import pandas as pd;
import numpy as np

result = np.array([
    [0.0039, 0.0203, 0.0432, 0.1322, 0.2085, 0.2889, 0.3389],
    [0.0078, 0.0413, 0.0988, 0.2992, 0.4729, 0.5926, 0.7595],
    [0.0176, 0.1092, 0.2154, 0.6003, 0.7965, 0.8881, 0.9437],
])

result = pd.DataFrame(result.T, index=[0.0001, 0.0005, 0.001, 0.003, 0.005, 0.007, 0.01], columns=[3, 5, 7])

print(result)

sns.set(style="whitegrid")

g = sns.lineplot(data=result)
g.set_ylabel("logical error rate")
g.set_xlabel("physical error rate")
g.set_yscale("log")
g.set_xscale("log")

plt.show()


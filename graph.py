from matplotlib.style import context
import seaborn as sns
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np

# result = np.array(
#    [
#        [0.0039, 0.0203, 0.0432, 0.1322, 0.2085, 0.2889, 0.3389],
#        [0.0078, 0.0413, 0.0988, 0.2992, 0.4729, 0.5926, 0.7595],
#        [0.0176, 0.1092, 0.2154, 0.6003, 0.7965, 0.8881, 0.9437],
#    ]
# )

result = np.array(
    [
        [0.0002, 0.0001, 0.0017, 0.0253, 0.0436, 0.0743, 0.106],
        [0.0, 0.0012, 0.0065, 0.0616, 0.1326, 0.2084, 0.3157],
        [0.0, 0.0026, 0.0154, 0.1584, 0.3034, 0.4028, 0.4693],
    ]
)

result = np.array(
    [
        [0.0, 0.0, 0.0, 0.0017, 0.0042, 0.0078, 0.0158],
        [0.0, 0.0, 0.0002, 0.0025, 0.01, 0.014, 0.0319],
        [0.0, 0.0, 0.0001, 0.0025, 0.017, 0.0395, 0.084],
    ]
)

result = np.array(
    [
        [0.0, 0.0, 0.0, 0.0016, 0.0041, 0.0072, 0.0138],
        [0.0, 0.0, 0.0006, 0.002, 0.0075, 0.0147, 0.0362],
        [0.0, 0.0, 0.0001, 0.0051, 0.0188, 0.0377, 0.0819],
        [0.0, 0.0001, 0.0002, 0.0178, 0.0581, 0.114, 0.2183],
        [0.0, 0.0008, 0.0023, 0.0602, 0.1586, 0.2713, 0.3756],
    ]
)
result = np.array(
    [
        [0.0, 0.0, 0.0, 0.0012, 0.0031, 0.005, 0.0111],
        [0.0, 0.0, 0.0001, 0.0004, 0.0018, 0.0034, 0.0084],
        [0.0, 0.0, 0.0, 0.0002, 0.0005, 0.0014, 0.0041],
        [0.0, 0.0, 0.0, 0.0, 0.0001, 0.0011, 0.0026],
        [0.0, 0.0, 0.0, 0.0, 0.0001, 0.0003, 0.0016],
    ]
)

result = np.array(
    [
        [0.0425, 0.0386, 0.0516, 0.0724, 0.0886, 0.0863, 0.1139],
        [0.0597, 0.0831, 0.1162, 0.1435, 0.1661, 0.1979, 0.2335],
        [0.0637, 0.1073, 0.1437, 0.1973, 0.2427, 0.2873, 0.3237],
        [0.0659, 0.1141, 0.1851, 0.2414, 0.305, 0.3535, 0.3897],
        [0.0647, 0.1276, 0.2004, 0.2793, 0.3473, 0.4079, 0.441],
    ]
)

result = np.array(
    [
        [0.0018, 0.0037, 0.0039, 0.0051, 0.0078, 0.0098, 0.0112],
        [0.0013, 0.0017, 0.0024, 0.003, 0.0039, 0.0063, 0.0067],
        [0.0001, 0.0005, 0.0008, 0.0018, 0.0022, 0.0034, 0.0044],
    ]
)

result = np.array(
    [
        [0.012, 0.0372, 0.0796, 0.1269, 0.1701, 0.2166, 0.2599, 0.3005],
        [0.0074, 0.039, 0.0908, 0.1665, 0.244, 0.3157, 0.3718, 0.4042],
        [0.0041, 0.0353, 0.1091, 0.2173, 0.3299, 0.3979, 0.4458, 0.4802],
    ]
)

result = np.array(
    [
        [0.0112, 0.0397, 0.0808, 0.1254, 0.1725, 0.22, 0.2616, 0.3024],
        [0.008, 0.0367, 0.0926, 0.1692, 0.2483, 0.3184, 0.3702, 0.4109],
        [0.0041, 0.0348, 0.1111, 0.2186, 0.3311, 0.3963, 0.4476, 0.4799],
        [0.0022, 0.0346, 0.1314, 0.2718, 0.3932, 0.455, 0.4897, 0.4969],
        [0.0012, 0.0316, 0.1548, 0.3227, 0.4452, 0.4923, 0.4932, 0.4973],
    ]
)
result = pd.DataFrame(
    result.T,
    index=[0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08],
    columns=["$d$ = 3", "$d$ = 5", "$d$ = 7", "$d$ = 9", "$d$ = 11"],
)

print(result)

# ig, ax = plt.subplots(figsize=(8, 6), dpi=400)

sns.set(rc={"mathtext.fontset": "cm"})
sns.set_style(style="ticks", rc={"xtick.direction": "in", "ytick.left": False})
sns.set_context("talk")


grid = sns.relplot(kind="line", data=result, markers=True)

# g = sns.lineplot(data=result, markers=True, ax=ax)
grid.set(yscale="log")
grid.set_ylabels("logical error rate")
grid.set_xlabels("physical error rate")
# g.set_yscale("log")
# g.set_xscale("log")
grid.fig.set_size_inches((10, 6))

grid.savefig("qec.svg")

# plt.show()

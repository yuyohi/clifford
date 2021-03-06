import networkx as nx
import matplotlib.pyplot as plt

from clifford.code import rotated_surface_code

from icecream import ic


def test__gen_measurement_qubit():
    test_code = rotated_surface_code.RotatedSurfaceCode(5)

    fig = plt.figure()
    ax = fig.add_subplot(111)
    pos = {n: n for n in test_code._qubit_network.nodes}
    labels = nx.get_node_attributes(test_code._qubit_network, "type")
    node_color = ["red" if node["type"] == "Z" else "dodgerblue" if node["type"] == "X" else "dimgray" for node in
                  test_code._qubit_network.nodes.values()]

    nx.draw(test_code._qubit_network, font_weight="bold", labels=labels, ax=ax, pos=pos, node_color=node_color)

    plt.show()


def test_syndrome_measurement():
    test_code = rotated_surface_code.RotatedSurfaceCode(5)

    error_map = test_code.syndrome_measurement(1)

    ic(error_map["X"].nodes())
    ic(len(error_map["X"].nodes()))
    ic(error_map["Z"].nodes())

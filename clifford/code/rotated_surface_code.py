from numpy import random
import networkx as nx

from clifford.simulator import simulator_chp


class RotatedSurfaceCode:
    def __init__(self, distance, p=0.1, seed=0):
        self._distance = distance
        if distance % 2 == 0:
            raise Exception("distance must be odd number")

        self._p = p

        self._code = simulator_chp.Simulator(distance ** 2 * 2 - 1, seed=seed)

        # measurement qubit の生成
        self._measurement_qubit = {"Z": [], "X": []}
        self._gen_measurement_qubit()

        # data qubit の生成
        self._data_qubit = [(x, y) for x in range(0, self._distance * 2, 2) for y in range(0, self._distance * 2, 2)]
        self._qubit_network = nx.Graph()

        # グラフとして表現
        self._qubit_network.add_nodes_from(self._measurement_qubit["Z"])
        nx.set_node_attributes(self._qubit_network, {coord: "Z" for coord in self._measurement_qubit["Z"]}, name="type")
        self._qubit_network.add_nodes_from(self._measurement_qubit["X"])
        nx.set_node_attributes(self._qubit_network, {coord: "X" for coord in self._measurement_qubit["X"]}, name="type")
        self._qubit_network.add_nodes_from(self._data_qubit)
        nx.set_node_attributes(self._qubit_network, {coord: "data" for coord in self._data_qubit}, name="type")
        # edgeをつなげる
        for u in self._qubit_network.nodes():
            if (v := (u[0] + 1, u[1] + 1)) in self._qubit_network.nodes():
                self._qubit_network.add_edge(u, v)
            if (v := (u[0] - 1, u[1] + 1)) in self._qubit_network.nodes():
                self._qubit_network.add_edge(u, v)

    def _gen_measurement_qubit(self):
        """
        generate measurement qubits

        """
        measurement_qubit = []
        for y in range(1, self._distance * 2 - 1, 4):
            for x in range(1, self._distance * 2, 2):
                measurement_qubit.append((x, y))
        for y in range(3, self._distance * 2 - 1, 4):
            for x in range(-1, self._distance * 2 - 1, 2):
                measurement_qubit.append((x, y))

        # 上下のqubitを追加
        for x in range(1, self._distance * 2 - 2, 4):
            measurement_qubit.append((x, -1))
        for x in range(3, self._distance * 2 - 2, 4):
            measurement_qubit.append((x, self._distance * 2 - 1))

        #  Z, Xに振り分ける
        qubit_is_Z = True
        for y in range(-1, self._distance * 2, 2):
            for x in range(-1, self._distance * 2, 2):
                if (x, y) in measurement_qubit:
                    if qubit_is_Z:
                        self._measurement_qubit["Z"].append((x, y))
                    else:
                        self._measurement_qubit["X"].append((x, y))
                    # 交互に入れ替える
                qubit_is_Z = not qubit_is_Z

            qubit_is_Z = not qubit_is_Z



from numpy import random
import networkx as nx
import retworkx as rx

from clifford.simulator import simulator_chp

from icecream import ic


class RotatedSurfaceCode:
    def __init__(self, distance, p=0.1, seed=0):
        self._distance = distance
        if distance % 2 == 0:
            raise Exception("distance must be odd number")

        self._p = p

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

        # decode用のグラフ (verticesがstabilizerで、edgeがdata qubit)
        self._stabilizer_graph = {"Z": nx.Graph(), "X": nx.Graph()}
        self._stabilizer_graph["Z"].add_nodes_from(self._measurement_qubit["Z"])
        self._stabilizer_graph["X"].add_nodes_from(self._measurement_qubit["X"])
        # for u in self._stabilizer_graph["Z"].nodes():

        # シミュレーション用
        self._sim = simulator_chp.Simulator(distance ** 2 * 2 - 1, seed=seed)
        self._coord_to_sim = {coord: n for n, coord in enumerate(self._qubit_network.nodes())}

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

    def syndrome_measurement(self, round=1) -> dict[str, rx.PyGraph]:
        x_order = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
        z_order = [(1, 1), (-1, 1), (1, -1), (-1, -1)]

        error_map = {"X": rx.PyGraph(), "Z": rx.PyGraph()}
        detection_point_X = []
        detection_point_Z = []

        for n in range(round):
            # xスタビライザーにHゲートを作用させる
            for x_stab in self._measurement_qubit["X"]:
                self._sim.h(self._coord_to_sim[x_stab])

            # CNOT
            for i in range(4):
                for z_stab in self._measurement_qubit["Z"]:  # Zスタビライザーについて
                    data_bit_coord = (z_stab[0] + z_order[i][0], z_stab[1] + z_order[i][1])
                    if data_bit_coord not in self._coord_to_sim:  # data bitがない場合
                        continue
                    self._sim.cx(self._coord_to_sim[data_bit_coord], self._coord_to_sim[z_stab])

                for x_stab in self._measurement_qubit["X"]:  # Xスタビライザーについて
                    data_bit_coord = (x_stab[0] + x_order[i][0], x_stab[1] + x_order[i][1])
                    if data_bit_coord not in self._coord_to_sim:  # data bitがない場合
                        continue
                    self._sim.cx(self._coord_to_sim[x_stab], self._coord_to_sim[data_bit_coord])

            # 再びxスタビライザーにHゲートを作用させる
            for x_stab in self._measurement_qubit["X"]:
                self._sim.h(self._coord_to_sim[x_stab])

            # measurement
            measurement_X = {coord: self._sim.measurement(self._coord_to_sim[coord]) for coord in
                             self._measurement_qubit["X"]}
            measurement_Z = {coord: self._sim.measurement(self._coord_to_sim[coord]) for coord in
                             self._measurement_qubit["Z"]}
            print(self._measurement_qubit["Z"])

            detection_point_X = list(map(lambda item: item[0] + (n,),
                                         filter(lambda item: item[1] == 1, measurement_X.items())))
            detection_point_Z = list(map(lambda item: item[0] + (n,),
                                         filter(lambda item: item[1] == 1, measurement_Z.items())))

        if n == 0:
            error_map["X"].add_nodes_from(detection_point_X)
            error_map["Z"].add_nodes_from(detection_point_Z)
        else:
            pass




        return error_map

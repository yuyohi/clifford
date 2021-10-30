from clifford import simulator_chp
import numpy as np


def test_simulator():
    test_sim = simulator_chp.Simulator(10)
    print(test_sim.__stabilizer_tableau)


def test():
    test = np.zeros((1, 2))
    print(test[5, 0])


def test_cx():
    qubit_num = 2
    sim = simulator_chp.Simulator(qubit_num)
    sim.cx(0, 1)
    sim.cx(0, 1)

    size = qubit_num * 2
    ans = np.concatenate((np.eye(size, dtype=int), np.zeros((size, 1), dtype=int)), axis=1)
    assert (sim.stabilizer_tableau == ans).all()

    sim.reset_tableau()
    sim._stabilizer_tableau[2, 4] = 1
    sim.cx(0, 1)

    ans_list = [[1, 1, 0, 0, 0], [0, 1, 0, 0, 0], [0, 0, 1, 0, 1], [0, 0, 1, 1, 0]]
    ans = np.array(ans_list)
    assert (sim.stabilizer_tableau == ans).all()


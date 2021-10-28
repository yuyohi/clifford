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

    size = qubit_num * 2
    ans = np.concatenate((np.eye(size, dtype=int), np.zeros((size, 1), dtype=int)), axis=1)
    print(ans)
    print(sim.stabilizer_tableau)
    assert (sim.stabilizer_tableau == ans).all()

from clifford import simulator_chp
import numpy as np


def test_simulator():
    test_sim = simulator_chp.Simulator(10)
    print(test_sim._stabilizer_tableau)


def test():
    test = np.eye(5)
    print(test[4, -1])


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


def test_measurement():
    qubit_num = 4
    sim = simulator_chp.Simulator(qubit_num, seed=3)

    assert sim.measurement(0) == 0
    assert sim.measurement(1) == 0
    assert sim.measurement(2) == 0
    assert sim.measurement(3) == 0

    sim.h(0)
    sim.h(1)
    sim.h(2)
    sim.h(3)
    result = [sim.measurement(0), sim.measurement(1), sim.measurement(2), sim.measurement(3)]
    print(result)

    result_2 = [sim.measurement(0), sim.measurement(1), sim.measurement(2), sim.measurement(3)]

    assert result == result_2

    sim.reset_tableau()
    sim.x(0)
    sim.x(1)
    sim.x(2)
    assert sim.measurement(0) == 1
    print([sim.measurement(0), sim.measurement(1), sim.measurement(2), sim.measurement(3)])


def test_make_bell_state():
    sim = simulator_chp.Simulator(2, seed=255)
    sim.h(0)
    sim.cx(0, 1)

    result = [sim.measurement(0), sim.measurement(1)]
    print(result)

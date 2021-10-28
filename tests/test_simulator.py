from clifford import  simulator

def test_simulator():
    test_sim = simulator.Simulator(10)
    print(test_sim._pauli_strings)

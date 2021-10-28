import numpy as np

class Simulator:
    def __init__(self, bit_num: int):
        self._pauli_strings = {"X": np.zeros((bit_num, bit_num), dtype=int), "Z": np.identity(bit_num, dtype=int)}





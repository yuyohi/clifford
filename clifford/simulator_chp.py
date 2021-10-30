import numpy as np


class Simulator:
    def __init__(self, qubit_num: int):
        self.qubit_num = qubit_num
        size = qubit_num * 2
        self._stabilizer_tableau = np.concatenate((np.eye(size, dtype=int), np.zeros((size, 1), dtype=int)), axis=1)

    def cx(self, a: int, b: int):
        """
        CNOT gate
        Args:
            a: control bit
            b: target bit

        """
        # rを計算
        self._stabilizer_tableau[:, self.qubit_num] ^= self._stabilizer_tableau[:, a] \
                                                       & self._stabilizer_tableau[:, self.qubit_num + b] \
                                                       & (self._stabilizer_tableau[:, b]
                                                          ^ self._stabilizer_tableau[:, self.qubit_num + a] ^ 1)

        # xを計算
        self._stabilizer_tableau[:, b] ^= self._stabilizer_tableau[:, a]
        # zを計算
        self._stabilizer_tableau[:, self.qubit_num + a] ^= self._stabilizer_tableau[:, self.qubit_num + b]

    def h(self, a: int):
        """
        Hadamard gate
        Args:
            a: target bit

        """
        self.stabilizer_tableau[:, self.qubit_num] ^= self._stabilizer_tableau[:, a] \
                                                      & self._stabilizer_tableau[:, self.qubit_num + a]

        self._stabilizer_tableau[:, a], self._stabilizer_tableau[:, self.qubit_num + a] \
            = self._stabilizer_tableau[:, self.qubit_num + a], self._stabilizer_tableau[:, a].copy()

    def s(self, a: int):
        """
        S gate (Phase gate)
        Args:
            a: target bit

        Returns:

        """
        self.stabilizer_tableau[:, self.qubit_num] ^= self._stabilizer_tableau[:, a] \
                                                      & self._stabilizer_tableau[:, self.qubit_num + a]

        self._stabilizer_tableau[:, self.qubit_num + a] ^= self.stabilizer_tableau[:, a]

    def reset_tableau(self):
        """
        Reset stabilizer tableau
        """
        size = self.qubit_num * 2
        self._stabilizer_tableau = np.concatenate((np.eye(size, dtype=int), np.zeros((size, 1), dtype=int)), axis=1)

    @property
    def stabilizer_tableau(self):
        return self._stabilizer_tableau

    @stabilizer_tableau.setter
    def stabilizer_tableau(self, value: np.ndarray):
        self._stabilizer_tableau = value

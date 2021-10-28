import numpy as np


class Simulator:
    def __init__(self, qubit_num: int):
        self.qubit_num = qubit_num
        size = qubit_num * 2
        self.__stabilizer_tableau = np.concatenate((np.eye(size, dtype=int), np.zeros((size, 1), dtype=int)), axis=1)

    def cx(self, a: int, b: int):
        # rを計算
        self.__stabilizer_tableau[:, self.qubit_num] ^= self.__stabilizer_tableau[:, a] \
                                                        & self.__stabilizer_tableau[:, self.qubit_num + b - 1] \
                                                        & (self.__stabilizer_tableau[:, b]
                                                           ^ self.__stabilizer_tableau[:, self.qubit_num + a - 1] ^ 1)

        # xを計算
        self.__stabilizer_tableau[:, b] ^= self.__stabilizer_tableau[:, a]
        # zを計算
        self.__stabilizer_tableau[:, a] ^= self.__stabilizer_tableau[:, b]

    @property
    def stabilizer_tableau(self):
        return self.__stabilizer_tableau

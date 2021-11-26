import numpy as np
from numpy import random

from icecream import ic


class Simulator:
    def __init__(self, qubit_num: int, seed=0):
        self._qubit_num = qubit_num
        size = qubit_num * 2
        self._stabilizer_tableau = np.concatenate((np.eye(size, dtype=int), np.zeros((size, 1), dtype=int)), axis=1)
        random.seed(seed)

    def cx(self, a: int, b: int):
        """
        CNOT gate
        Args:
            a: control bit
            b: target bit

        """
        # rを計算
        self._stabilizer_tableau[:, -1] ^= self._stabilizer_tableau[:, a] \
                                           * self._stabilizer_tableau[:, self._qubit_num + b] \
                                           * (self._stabilizer_tableau[:, b]
                                              ^ self._stabilizer_tableau[:, self._qubit_num + a] ^ 1)

        # xを計算
        self._stabilizer_tableau[:, b] ^= self._stabilizer_tableau[:, a]
        # zを計算
        self._stabilizer_tableau[:, self._qubit_num + a] ^= self._stabilizer_tableau[:, self._qubit_num + b]

    def h(self, a: int):
        """
        Hadamard gate
        Args:
            a: target bit

        """
        self._stabilizer_tableau[:, -1] \
            ^= self._stabilizer_tableau[:, a] * self._stabilizer_tableau[:, self._qubit_num + a]

        self._stabilizer_tableau[:, a], self._stabilizer_tableau[:, self._qubit_num + a] \
            = self._stabilizer_tableau[:, self._qubit_num + a], self._stabilizer_tableau[:, a].copy()

    def s(self, a: int):
        """
        S gate (Phase gate)
        Args:
            a: target bit

        """
        self.stabilizer_tableau[:, -1] \
            ^= self._stabilizer_tableau[:, a] * self._stabilizer_tableau[:, self._qubit_num + a]

        self._stabilizer_tableau[:, self._qubit_num + a] ^= self.stabilizer_tableau[:, a]

    def x(self, a: int):
        """
        X gate
        Args:
            a: target bit

        """
        self._stabilizer_tableau[:, -1] ^= self._stabilizer_tableau[:, self._qubit_num + a]

    def z(self, a: int):
        """
        Z gate
        Args:
            a: target bit

        """
        self._stabilizer_tableau[:, -1] ^= self._stabilizer_tableau[:, a]

    def _g(self, x1: int, z1: int, x2: int, z2: int) -> int:
        if x1 == z1 == 0:
            return 0
        elif x1 == z1 == 1:
            return z2 - x2
        elif x1 == 1 and z1 == 0:
            return z2 * (2 * x2 - 1)
        elif x1 == 0 and z1 == 1:
            return x2 * (1 - 2 * z2)
        else:
            raise Exception("tableau error: parameters must be 0 or 1")

    def _row_sum(self, h: int, i: int):
        g_sum = 0
        for j in range(self._qubit_num):
            g_sum += self._g(self._stabilizer_tableau[i, j], self._stabilizer_tableau[i, self._qubit_num + j],
                             self._stabilizer_tableau[h, j], self._stabilizer_tableau[h, self._qubit_num + j])

        checker = 2 * self._stabilizer_tableau[h, -1] + 2 * self._stabilizer_tableau[i, -1] + g_sum
        if checker % 4 == 2:
            self._stabilizer_tableau[h, -1] = 1
        elif checker % 4 == 0:
            self._stabilizer_tableau[h, -1] = 0
        else:
            ic(checker)
            ic(g_sum)
            raise Exception("error")

        self._stabilizer_tableau[h, :-1] ^= self._stabilizer_tableau[i, :-1]

    def _row_sum_temp(self, i: int, temp: np.ndarray):
        g_sum = 0
        for j in range(self._qubit_num):
            g_sum += self._g(self._stabilizer_tableau[i, j], self._stabilizer_tableau[i, self._qubit_num + j],
                             temp[0, j], temp[0, self._qubit_num + j])

        checker = 2 * temp[0, -1] + 2 * self._stabilizer_tableau[i, -1] + g_sum
        if checker % 4 == 2:
            temp[0, -1] = 1
        elif checker % 4 == 0:
            temp[0, -1] = 0
        else:
            raise Exception("error")

        temp[0, :-1] ^= self._stabilizer_tableau[i, :-1]

    def measurement(self, a: int) -> int:
        outcome_is_random = False
        p = []
        for i in range(self._qubit_num, self._qubit_num * 2):
            if self._stabilizer_tableau[i, a] == 1:
                outcome_is_random = True
                p.append(i)

        # 一つでもXpa = 1のとき、結果はランダム
        if outcome_is_random:
            # 最初のp以外を置換
            for i in p[1:]:
                self._row_sum(i, p[0])
            # (p[0] - qubit_num) 行目をp[0]行目に置換
            self._stabilizer_tableau[p[0] - self._qubit_num, :] = self._stabilizer_tableau[p[0], :]
            self._stabilizer_tableau[p[0], :] = np.zeros((1, self._stabilizer_tableau.shape[1]), dtype=int)
            self._stabilizer_tableau[p[0], self._qubit_num + a] = 1

            # rpを1/2でセットしこれが観測結果となる
            if random.rand() < 0.5:
                self._stabilizer_tableau[p[0], -1] = 1
            else:
                self._stabilizer_tableau[p[0], -1] = 0

            return self._stabilizer_tableau[p[0], -1]

        else:  # 測定結果が決定的のとき
            temp_space = np.zeros((1, self._stabilizer_tableau.shape[1]), dtype=int)
            for i in range(self._qubit_num):
                if self._stabilizer_tableau[i, a] == 1:
                    self._row_sum_temp(i + self._qubit_num, temp_space)

            return temp_space[0, -1]

    def reset_tableau(self):
        """
        Reset stabilizer tableau
        """
        size = self._qubit_num * 2
        self._stabilizer_tableau = np.concatenate((np.eye(size, dtype=int), np.zeros((size, 1), dtype=int)), axis=1)

    @property
    def stabilizer_tableau(self):
        return self._stabilizer_tableau

    @stabilizer_tableau.setter
    def stabilizer_tableau(self, value: np.ndarray):
        self._stabilizer_tableau = value

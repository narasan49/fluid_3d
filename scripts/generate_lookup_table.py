"""
https://www.cs.upc.edu/~virtual/SGI/docs/1.%20Theory/Unit%2010.%20Volume%20models.%20Marching%20Cubes/Marching%20Cubes.pdf
を参考にMarching CubesのLookup tableを作成
"""

import numpy as np
import copy

# Cubeの各辺をx軸に回転したときのインデックス遷移
rx_edge = np.array([8, 4, 5, 0, 9, 10, 1, 2, 11, 6, 7, 3])
# 同 y軸
ry_edge = np.array([4, 9, 1, 6, 8, 0, 11, 3, 5, 10, 2, 7])
# 同 z軸
rz_edge = np.array([2, 0, 3, 1, 5, 7, 4, 6, 10, 8, 11, 9])

# Cubeの各頂点を各軸で回転したときのインデックス遷移
rx_vert = np.array([4, 5, 0, 1, 6, 7, 2, 3])
ry_vert = np.array([4, 0, 6, 2, 5, 1, 7, 3])
rz_vert = np.array([1, 3, 0, 2, 5, 7, 4, 6])

# 辺のインデックスを頂点の組に変換するテーブル
idx_to_edge = np.array(["[0, 1]", "[0, 2]", "[1, 3]", "[2, 3]", "[0, 4]", "[1, 5]", "[2, 6]", "[3, 7]", "[4, 5]", "[4, 6]", "[5, 7]", "[6, 7]"])

class Triangle:
    """
    辺のインデックス(0-11)で三角形を構成する
    """
    def __init__(self, e0, e1, e2):
        self.e0 = e0
        self.e1 = e1
        self.e2 = e2
    def rot(self, i, j, k):
        """
        三角形の各辺を
        x軸にi*90°, y軸にj*90°, z軸にk*90°
        順に回転する。
        """
        for _ in range(i):
            self.e0 = rx_edge[self.e0]
            self.e1 = rx_edge[self.e1]
            self.e2 = rx_edge[self.e2]
        for _ in range(j):
            self.e0 = ry_edge[self.e0]
            self.e1 = ry_edge[self.e1]
            self.e2 = ry_edge[self.e2]
        for _ in range(k):
            self.e0 = rz_edge[self.e0]
            self.e1 = rz_edge[self.e1]
            self.e2 = rz_edge[self.e2]
    def rust_snippet(self):
        """
        三角形のインスタンスを構築するrustコードを返す
        """
        return f"EdgeTriangle::new([{idx_to_edge[self.e0]}, {idx_to_edge[self.e1]}, {idx_to_edge[self.e2]}])"
    

class CubeBits:
    """
    Cubeの頂点の内側/外側を表現するビット列
    外側頂点のインデックスを保持する
    """
    def __init__(self, bits):
        self.bits = bits
    def rot(self, i, j, k):
        for bi in range(len(self.bits)):
            for _ in range(i):
                self.bits[bi] = rx_vert[self.bits[bi]]
            for _ in range(j):
                self.bits[bi] = ry_vert[self.bits[bi]]
            for _ in range(k):
                self.bits[bi] = rz_vert[self.bits[bi]]
    def idx(self):
        sum = 0
        for i in range(len(self.bits)):
            sum += (1 << self.bits[i])
        return sum

numbers = ["one", "two", "three", "four", "five"]

def triangles_snippet(triangles):
    i = len(triangles)

    return f"EdgeTriangles::{numbers[i - 1]}([{",".join(map(lambda t: t.rust_snippet(), triangles))}])"

# 頂点の内外パターンを回転・内外反転させて、長さ256のLookup tableを構築
elements = np.full(256, "", dtype=object)
# 頂点パターン
class Pattern:
    def __init__(self, rot_pattern, bits, triangles):
        self.rot_pattern = rot_pattern
        self.bits = bits
        self.triangles = triangles

patterns = [
    # パターン1: 1頂点のみ外側
    Pattern(
        [[i, j, k] for i in range(2) for j in range(1) for k in range(4)],
        [0],
        [Triangle(0, 4, 1)]
    ),
    # パターン1-b: 1頂点のみ内側
    Pattern(
        [[i, j, k] for i in range(2) for j in range(1) for k in range(4)],
        [1, 2, 3, 4, 5, 6, 7],
        [Triangle(0, 1, 4)]
    ),
    # パターン2
    Pattern(
        [[i, j, k] for i in range(1) for j in range(3) for k in range(4)],
        [0, 1],
        [Triangle(1, 5, 4), Triangle(5, 1, 2)]
    ),
    # パターン2-b
    Pattern(
        [[i, j, k] for i in range(1) for j in range(3) for k in range(4)],
        [2, 3, 4, 5, 6, 7],
        [Triangle(1, 4, 5), Triangle(5, 2, 1)]
    ),
    # パターン3
    Pattern(
        [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 1, 0], [0, 1, 1], [0, 1, 2], [0, 1, 3], [1, 0, 0], [1, 0, 1], [3, 0, 0], [3, 0, 1]],
        [0, 5],
        [Triangle(1, 8, 4), Triangle(8, 1, 10), Triangle(0, 10, 1), Triangle(10, 0, 5)]
    ),
    # パターン3-b: 頂点の内外は反転するが、面の作り方が異なる。
    Pattern(
        [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 1, 0], [0, 1, 1], [0, 1, 2], [0, 1, 3], [1, 0, 0], [1, 0, 1], [3, 0, 0], [3, 0, 1]],
        [1, 2, 3, 4, 6, 7],
        [Triangle(0, 1, 4), Triangle(5, 8, 10)]
    ),
    # パターン4
    Pattern(
        [[0, 0, k] for k in range(4)],
        [0, 7],
        [Triangle(0, 4, 1), Triangle(7, 11, 10)]
    ),
    # パターン4-b
    Pattern(
        [[0, 0, k] for k in range(4)],
        [1, 2, 3, 4, 5, 6],
        [Triangle(0, 1, 4), Triangle(7, 10, 11)]
    ),
    # パターン5
    Pattern(
        [[i, j, k] for i, j in [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [1, 3]] for k in range(4)],
        [1, 2, 3],
        [Triangle(0, 1, 5), Triangle(5, 1, 6), Triangle(5, 6, 7)]
    ),
    # パターン5-b
    Pattern(
        [[i, j, k] for i, j in [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [1, 3]] for k in range(4)],
        [0, 4, 5, 6, 7],
        [Triangle(0, 5, 1), Triangle(5, 6, 1), Triangle(5, 7, 6)]
    ),
    # パターン6
    Pattern(
        [[i, j, k] for i, j in [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [1, 3]] for k in range(4)],
        [0, 1, 7],
        [Triangle(1, 2, 7), Triangle(1, 2, 7), Triangle(4, 1, 11), Triangle(4, 10, 5), Triangle(4, 11, 10)]
    ),
    # パターン6-b: 頂点の内外は反転するが、面の作り方が異なる。
    Pattern(
        [[i, j, k] for i, j in [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [1, 3]] for k in range(4)],
        [2, 3, 4, 5, 6],
        [Triangle(1, 4, 5), Triangle(1, 5, 2), Triangle(10, 11, 7)]
    ),
    # パターン7
    Pattern(
        [[i, j, k] for i in range(1) for j in range(2) for k in range(4)],
        [1, 4, 7],
        [Triangle(4, 11, 9), Triangle(4, 0, 11), Triangle(7, 11, 0), Triangle(7, 0, 2), Triangle(5, 8, 10)]
    ),
    # パターン7-b: 頂点の内外は反転するが、面の作り方が異なる。
    Pattern(
        [[i, j, k] for i in range(1) for j in range(2) for k in range(4)],
        [0, 2, 3, 5, 6],
        [Triangle(0, 5, 2), Triangle(4, 9, 8), Triangle(7, 10, 11)]
    ),
    # パターン8
    Pattern(
        [[0, 0, 0], [1, 0, 0], [1, 0, 1], [1, 0, 2], [1, 0, 3], [2, 0, 0]],
        [0, 1, 2, 3],
        [Triangle(4, 6, 5), Triangle(5, 6, 7)]
    ),
    # パターン9
    Pattern(
        [[i, 0, k] for i in [0, 2] for k in range(4)],
        [0, 2, 3, 6],
        [Triangle(0, 4, 9), Triangle(0, 9, 2), Triangle(2, 9, 11), Triangle(2, 11, 7)]
    ),
    # パターン10
    Pattern(
        [[0, 0, 0], [1, 0, 0], [1, 0, 1], [1, 0, 2], [1, 0, 3], [2, 0, 0]],
        [0, 3, 4, 7],
        [Triangle(0, 8, 10), Triangle(0, 10, 2), Triangle(1, 11, 9), Triangle(1, 3, 11)]
    ),
    # パターン11
    Pattern(
        [[0, j, k] for j in range(3) for k in range(4)],
        [0, 2, 3, 7],
        [Triangle(0, 4, 6), Triangle(0, 6, 2), Triangle(2, 6, 11), Triangle(2, 11, 7)]
    ),
    # パターン12
    Pattern(
        [[i, j, k] for i, j in [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [1, 3]] for k in range(4)],
        [0, 2, 3, 5],
        [Triangle(0, 5, 2), Triangle(6, 8, 4), Triangle(6, 10, 8), Triangle(6, 7, 10)]
    ),
    # パターン13
    Pattern(
        [[0, 0, 0], [0, 0, 1]],
        [0, 3, 5, 6],
        [Triangle(0, 5, 2), Triangle(1, 3, 6), Triangle(4, 9, 8), Triangle(7, 10, 11)]
    ),
    # パターン14
    Pattern(
        [[i, 0, k] for i in range(3) for k in range(4)],
        [0, 2, 3, 4],
        [Triangle(0, 8, 9), Triangle(0, 9, 6), Triangle(0, 6, 2), Triangle(2, 6, 7)]
    )
]

elements[0] = "EdgeTriangles::ZERO"
elements[255] = "EdgeTriangles::ZERO"
for pattern in patterns:
    for i, j, k in pattern.rot_pattern:
        cube_bits = CubeBits(pattern.bits.copy())
        triangles = copy.deepcopy(pattern.triangles)
        cube_bits.rot(i, j, k)
        for t in triangles:
            t.rot(i, j, k)
        elements[cube_bits.idx()] = triangles_snippet(triangles)

print("pub const LUT: [EdgeTriangles; 256] = [")
print(",\n".join(elements))
print("];")
# for idx, e in enumerate(elements):
#     print(idx, e)
from random import randint

class lineDataset:
    def __init__(self, qnts_resultados, grade, function):
        self.final = list()
        for _ in range(0, qnts_resultados + 1):
            self.final.append(self.algebric(grade, function))

    def algebric(self, len, algebra):
        while True:
            x = randint(len[0], len[1])
            y = algebra(x)
            if y >= len[0] and y <= len[1]:
                return [x, y]



func = lambda x: x*3 + 2
abc = lineDataset(7, [-6, 6], func)
print(abc.final)
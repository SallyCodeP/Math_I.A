
def multi_matriz(m1=list(), m2=list()):
    if check_m_len(m1) and check_m_len(m2) and len(m1[0]) == len(m2):
        final = list()

        # Capturando colunas de "m2"
        colunas_m2 = [[] for _ in range(0, len(m2[0]))]
        for linha in m2:
            for idx, value in enumerate(linha):
                colunas_m2[idx].append(value)

        capsula = list()
        for l in m1:
            for ele in colunas_m2:
                add = [l[n] * ele[n] for n in range(0, len(ele))]
                a = 0
                for j in add:
                    a += j
                capsula.append(a)
            final.append(capsula)
            capsula = []
        return final
    else:
        print("Erro")


def check_m_len(m1, m2 = False):
    tamanho = len(m1[0])
    for element in m1:
        if tamanho != len(element) or tamanho <= 0:
            return False
    if not m2:
        return True
    else:
        for element in m2:
            if tamanho != len(element):
                return False
    return True


def get_determinant(m1):
    if is_m_square(m1) and check_m_len(m1):
        if len(m1) == 2:
            return (m1[0][0] * m1[1][1]) - (m1[0][1] * m1[1][0])
        elif len(m1) == 3:
            return __3x3(m1)
        else:
            return __3x3(__transform_in_3x3(m1))
    else:
        print("Erro")


def is_m_square(m1):
    for element in m1:
        if len(element) != len(m1):
            return False
    return True



def __3x3(m1):
    '''Acha qualquer determinante de uma matriz de ordem 3'''
    if len(m1) == 3 and check_m_len(m1) and is_m_square(m1):
        soma = (m1[0][0] * m1[1][1] * m1[2][2]) + ((m1[0][1] * m1[1][2]) * m1[2][0]) + ((m1[0][2] * m1[1][0]) * m1[2][1])
        sub = ((m1[0][2] * m1[1][1]) * m1[2][0])  + ((m1[0][1] * m1[1][0]) * m1[2][2]) + ((m1[0][0] * m1[1][2]) * m1[2][1])
        return soma - sub


def __transform_in_3x3(m1):
    '''
    Usa o teorema de Jacob e a regra de Chió
    para transformar qualquer matriz com ordem maior que 3 
    em uma matriz de ordem 3
    '''
    if check_m_len(m1) and is_m_square(m1):
        while len(m1) != 3:
            if m1[0][0] != 1:
                m1 = __jacob(m1)
            else:
                m1 = [__chio(e, m1[0]) for idx, e in enumerate(m1) if idx != 0]
        return m1


def __jacob(m1):
    distancia = m1[0][0] - 1
    for i, e in enumerate(m1[0]):
        if distancia % e == 0 and i != 0:
            x = distancia / e
            diminu = [a[i] * x for a in m1]
            print(e)
            print(diminu)
            for num in range(0, len(m1)):
                m1[num][0] = int(m1[num][0] - diminu[num])
            return m1

    # Resolver por eleminação de Gauss

    


def __chio(new, old):
    if old[0] == 1 and len(new) == len(old):
        repasse = list()
        for i, e in enumerate(new):
            if i != 0:
                count = e - (old[i] * new[0])
                repasse.append(count) 
        return repasse

c = [
    [1, 2, 5, 2],
    [3, 7, 8, 1],
    [8, 2, 6, 5],
    [3, 4, 9, 4]
]

m1 = [[6.0, 7.0, 2.0], [6.0, 5.0, 3.0], [1.0, 7.0, 3.0]]
m2 = [[6.0, 8.0, 9.0], [6.0, 4.0, 3.0], [2.0, 7.0, 1.0]]
print(multi_matriz(m1, m2))

print(get_determinant(c))


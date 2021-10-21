


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
        if tamanho != len(element):
            return False
    if not m2:
        return True
    else:
        for element in m2:
            if tamanho != len(element):
                return False
    return True



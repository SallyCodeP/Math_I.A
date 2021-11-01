
    
    
    


            

def idx_menor_float(m1):
    idx = 0
    for i, a in enumerate(m1):
        if len(str(a)) < m1[idx]:
            idx = i
    return idx

c = [
    [9, 3, 3, 3],
    [3, 7, 8, 1],
    [8, 2, 6, 5],
    [3, 4, 9, 4]
] 

print(Jacob(c))

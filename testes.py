
def foward(x, w):
    final = []
    for e in x:
        final.append(e*w)
    return final

def loss(y, y_pred):
    f = sub_tensor(y_pred, y)
    got_elevate = elevate_all_for(f, 2.0)
    return sum_all(got_elevate)/len(y)

def div_for_all(x, num):
    final = []
    for e in x:
        final.append(e/num)
    return final

def mul_for_all(x, num):
    final = []
    for e in x:
        final.append(e*num)
    return final

def sum_all(x):
    final = 0
    for e in x:
        final += e
    return final

def elevate_all_for(x, num):
    final = []
    for e in x:
        final.append(e**num)
    return final

def mul_for_tensor(x, y):
    final = []
    for i, e in enumerate(x):
        final.append(e*y[i])
    return final

def sub_tensor(x, y):
    final = []
    for i, e in enumerate(x):
        final.append(e-y[i])
    return final

def grad(x, y_pred, y):
    mul = mul_for_tensor(sub_tensor(y_pred, y), mul_for_all(x, 2.0))
    a = sum_all(mul)
    return a/len(y)

def get_rate(repeat):
    if repeat > 0:
        num = len(str(repeat)) - 1
        final = ["0" for _ in range(num)]
        final.append("1")
        final = "".join(final)
        return float(f"0.{final}")
    else:
        print("Value error")

repeticao = 50
rate = get_rate(repeticao)
w = 1.0
x = [2.0, 3.0, 4.0, 5.0, 6.0]
y = [4.0, 6.0, 8.0, 10.0, 12.0]


for aprender in range(repeticao):
    y_pred = foward(x, w)
    print("-=" * 30)
    l = loss(y, y_pred)
    wd = grad(x, y_pred, y)
    print(f"Resultado loss {l}, resultado w*x {y_pred}")
    w -= wd * rate

print(f"final weights {w:.3f}")

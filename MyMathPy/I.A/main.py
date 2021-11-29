import numpy as np
import matplotlib.pyplot as plt

def function(x):
    return float(x*2)



x = np.array([2.0, 3.0, 4.0, 5.0, 6.0], dtype=np.float32)
y = np.array([4.0, 6.0, 8.0, 10.0, 12.0], dtype=np.float32)
w = 0.0



def foward(x):
    return x*w

print("Resultado inicial:")
print("-=" * 30)
print(foward(14))
print("-=" * 30)


def loss(y, y_pred):
    return ((y_pred - y)**2).mean()

def grad(x, y, y_pred):
    # usando regra da corrente para derivar w em loss
    # e depois pegando a media dos valores
    return np.dot(2*x, y_pred - y).mean()



repeticao = 50

def get_rate(repeat):
    if repeat > 0:
        num = len(str(repeat)) - 1
        final = ["0" for _ in range(num)]
        final.append("1")
        final = "".join(final)
        return float(f"0.{final}")
    else:
        print("Value error")
        
rate = get_rate(repeticao)

for aprender in range(repeticao):
    y_pred = foward(x)
    l = loss(y, y_pred)
    wd = grad(x, y, y_pred)
    print(f"Resultado loss {l}, resultado w*x {y_pred}")
    w -= wd * rate



print("-=" * 30)
print("Resultado final:")
print(f"{foward(14):.3f}")

m = [w*a for a in x]

plt.plot(x, y, "ro")
# plt.plot(X, pre, "b")
plt.show()
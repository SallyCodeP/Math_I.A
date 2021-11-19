import numpy as np 

def function(x):
    return float(x*2)



x = np.array([2.0, 3.0, 4.0, 5.0, 6.0], dtype=np.float32)
y = np.array([4.0, 6.0, 8.0, 10.0, 12.0], dtype=np.float32)
w = 0.0



def foward(x):
    return x*w

print(foward(14))

def loss(y, y_pred):
    return ((y_pred - y)**2).mean()

def grad(x, y, y_pred):
    # usando regra da corrente para derivar w em loss
    # e depois pegando a media dos valores
    return np.dot(2*x, y_pred - y).mean()


repeticao = 800

def get_rate(repeat):
    if repeat > 0:
        num = len(str(repeat)) - 1
        final = ["0" for _ in range(num)]
        final.append("1")
        final = "".join(final)
        return float(f"0.{final}")
    else:
        print("Value error")

print("rate")
rate = get_rate(repeticao)
print(rate)

for aprender in range(repeticao):
    y_pred = foward(x)
    l = loss(y, y_pred)
    wd = grad(x, y, y_pred)
    w -= wd * rate

print(f"{foward(14):.3f}")
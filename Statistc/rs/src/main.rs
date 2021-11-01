fn main() {
    R_linear(5.0, vec![[1.0, 2.0], [2.0, 3.0], [3.0, 4.0], [4.0, 5.0], [19.0, 20.0]]);
    let a = EMQ(vec![1.0, 2.0, 3.0, 19.0]);
    println!("{}", a);
}

fn EMQ(data: Vec<f64>) -> f64{
    let media = Media(&data);
    let mut penalidade: Vec<f64> = vec![];
    for dn in data.iter() {
        let calculo = media - dn;
        if calculo >= 0.0 {
            penalidade.push(calculo);
        }
        else {
            penalidade.push(calculo * -1.0);
        }
    }
    println!("{:?}", penalidade);
    return {
        Sum_all(&penalidade)
    };
}

fn Sum_all(data: &Vec<f64>) -> f64 {
    let mut soma = 0.0;
    for elements in data.iter() {
        soma += elements;
    }
    return soma;
}

fn Media(data: &Vec<f64>) -> f64 {
    return {
        let mut sum = 0.0;
        for element in data.iter() {
            sum += element;
        }
        let len = data.len() as f64;
        sum / len
    }
}

fn R_linear(myx: f64, data: Vec<[f64;2]>) -> f64 {
    /* Em "data" os dados devem estar organizados da seguinte forma -> [x, y] */
    let mut x = 0.0;
    let mut y = 0.0;
    let mut xy = 0.0;
    let mut x2 = 0.0;

    // capturando dados  
    for an in data.iter() {
        x += an[0];
        y += an[1];
        xy += an[0]*an[1];
        x2 += an[0].powf(2.0);
    }
    
    let n = data.len() as f64;
    let media_x = x /  n;
    let media_y = y / n;

    let beta = {
        let first = xy - {n * media_x * media_y};
        let second = x2 - {n * media_x.powf(2.0)};
        first / second
    };

    let alpha = media_y - {beta * media_x};

    return alpha + {beta * myx};
}


pub fn tensor_multi(x1: &Vec<f64>, x2: &Vec<f64>) -> Vec<f64>{
    let f_return: Vec<f64> = vec![];
    for (i, e) in x1.iter().enumerate() {
        f_return.push(e * x2[i]);
    }
    return f_return;
}

pub fn tensor_elevate(x1: &Vec<f64>, elevate: f64) -> &Vec<f64> {
    let f_return: Vec<f64> = vec![];
    for e in x1.iter() {
        f_return.push(e.powf(elevate))
    }
    return f_return;
}

pub fn tensor_sub(x1: &Vec<f64>, x2: &Vec<f64>) -> Vec<f64>{
    let f_return: Vec<f64> = vec![];
    for (i, e) in x1.iter().enumerate() {
        f_return.push(e - x2[i]);
    }
    return f_return;
}

pub fn tensor_media(data: &Vec<f64>) -> f64 {
    return {
        let mut sum = 0.0;
        for element in data.iter() {
            sum += element;
        }
        let len = data.len() as f64;
        sum / len
    }
}

pub fn tensor_EMQ(data: Vec<f64>) -> f64{
    let media = tensor_media(&data);
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
        tensor_sum_all(&penalidade)
    };
}

pub fn tensor_sum_all(data: &Vec<f64>) -> f64 {
    let mut soma = 0.0;
    for elements in data.iter() {
        soma += elements;
    }
    return soma;
}
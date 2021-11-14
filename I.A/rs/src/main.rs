fn main() {
    println!("Hello, world!");
    foward(x: &Vec<f64>, w: &Vec<f64>);
    
}

fn foward(x: &Vec<f64>, w: &Vec<f64>) -> Vec<f64> {
    return tensors_multi(x, w);
}

fn tensors_multi(x1: &Vec<f64>, x2: &Vec<f64>) -> Vec<f64>{
    let f_return: Vec<f64> = vec![];
    for (i, e) in x1.iter().enumerate() {
        f_return.push(e * x2[i]);
    }
    return f_return;
}

fn loss(y: &Vec<f64>, y_pred: &Vec<f64>) {
    {1.0/y.len() as f64} * {y}
}
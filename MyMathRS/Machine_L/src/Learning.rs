mod Tensor_calc;


fn foward(x: &Vec<f64>, w: &Vec<f64>) -> Vec<f64> {
    return tensors_multi(x, w);
}


fn loss(y: &Vec<f64>, y_pred: &Vec<f64>) {
    let sub = Tensor_calc.tensor_sub(y_pred, y);
    let elevate = Tensor_calc.tensor_elevate(sub, 2.0);
    return Tensor_calc.tensor_media(elevate)
}


fn grad(x: &Vec<f64>, y: &Vec<f64>, pred_y: &Vec<f64>) {
    mulx = Vec<f64> = vec![];
    for e in x.iter() {
        mulx.push(e*2); 
    }
    let fist = {
        let a = tensor_sub(pred_y, y);
        Tensor_calc.tensors_multi(a, mulx)
    }
    return Tensor_calc.tensor_media(first);
        
    

}

fn training(x: &Vec<f64>, y: &Vec<f64>, w: &Vec<f64>, times: i64) {
    learning_rate = 0.01;

    for ele in 1..= time {
        y_pred = foward(x, w);
        l = loss(y, y_pred);
        dw = grad(x, y, pred_y);
        w -= dw * learning_rate; 
    }

    let res_final = foward(x, w);
    print(res_final);
    
} 
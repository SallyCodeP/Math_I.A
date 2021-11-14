mod Tensor_calc;


fn foward(x: &Vec<f64>, w: &Vec<f64>) -> Vec<f64> {
    return tensors_multi(x, w);
}


fn loss(y: &Vec<f64>, y_pred: f64) {
    let sub = Tensor_calc.tensor_sub(y_pred, y);
    let elevate = Tensor_calc.tensor_elevate(sub, 2.0);
    return Tensor_calc.tensor_media(elevate)
}

loss/dy_pred * y_pred/dw

1/n * [(wx - y)**2]
(wx - y)**2


fn grad() {

}
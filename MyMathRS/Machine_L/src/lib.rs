mod Tensor_calc;


mod Learn {
    fn foward(x: &Vec<f64>, w: &Vec<f64>) -> Vec<f64> {
        return Tensor_calc.tensor_multi(x, w);
    }


    fn loss(y: &Vec<f64>, y_pred: &Vec<f64>) {
        let sub = Tensor_calc.tensor_sub(y_pred, y);
        let elevate = Tensor_calc.tensor_elevate(sub, 2.0);
        return Tensor_calc.tensor_media(elevate)
    }


    fn grad(x: &Vec<f64>, y: &Vec<f64>, pred_y: &Vec<f64>) {
        let mulx = Tensor_calc.tensor_mul_all(x, 2.0)
        let fist = {
            let a = tensor_sub(pred_y, y);
            Tensor_calc.tensors_multi(a, mulx)
        };
        return Tensor_calc.tensor_media(first);
    }

    fn training(x: &Vec<f64>, y: &Vec<f64>, w: &Vec<f64>, time: i64) {
        let learning_rate = 0.01;

        for ele in 1..= time {
            let y_pred = foward(x, w);
            let l = loss(y, y_pred);
            let dw = grad(x, y, pred_y);
            w -= dw * learning_rate; 
        }

        let res_final = foward(x, w);
        print(res_final);
        
    } 
}


mod Tensor;

struct Learn {
    x_data: Tensor::Tensor,
    y_data: Tensor::Tensor,
    w: f64
}

impl Learn {
    fn foward_Tensor(&self) -> Tensor::Tensor {
        return self.x_data.mul_all_for(self.w);
    }

    fn loss(&self) -> f64{
        let y_pred = self.foward_Tensor();
        let sub = self.y_data.sub(&y_pred);
        let elevate = sub.elevate_to(2.0);
        return elevate.media();
    }

    fn foward_f64(&self, x: f64) -> f64 {
        return self.w*x;
    }
    
    fn grad(&self) -> f64{
        let pred_y = self.foward_Tensor();
        let mulx = self.x_data.mul_all_for(2.0);
        let sub_a = self.y_data.sub(&pred_y);

        let f_return = mulx * sub_a;
        return f_return.media();
    }
}










pub fn training(x: &Vec<f64>, y: &Vec<f64>, w: f64, time: i64)-> f64 {
    let learning_rate = 0.01;

    for ele in 1..= time {
        let y_pred = foward_Vec(x, w);
        let l = loss(y, &y_pred);
        let dw = grad(x, y, &y_pred);
        w -= dw * learning_rate; 
    }
    return w;
    
} 
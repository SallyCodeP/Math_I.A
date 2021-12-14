
use crate::Tensor_mod::Tensor;

pub struct Learn {
    x_data: Tensor,
    y_data: Tensor,
    w: f64
}

impl Learn {

    pub fn foward_Tensor(&self) -> Tensor {
        return self.x_data.mul_all_for(self.w);
    }

    pub fn loss(&self) -> f64{
        let y_pred = self.foward_Tensor();
        let sub = self.y_data.sub(&y_pred);
        let elevate = sub.elevate_to(2.0);
        return elevate.media();
    }

    pub fn foward_f64(&self, x: f64) -> f64 {
        return self.w*x;
    }
    
    fn grad(&self) -> f64{
        let pred_y = self.foward_Tensor();
        let mulx = self.x_data.mul_all_for(2.0);
        let sub_a = self.y_data.sub(&pred_y);
        let f_return = mulx.mul_tesors(&sub_a);
        return f_return.media();
    }

    fn get_rate(&self, repeat: i64) -> f64{
        let num = repeat.to_string();
        let mut return_f = "0.".to_owned();
        for _ in 0.. (num.len() - 1) {
            return_f.push_str("0");
        }
        return_f += "1";
        return return_f.parse().unwrap();
    }

    pub fn training(&mut self, time: i64)-> f64 {
        let learning_rate = self.get_rate(time);

        for _ in 1..= time {
            let dw = self.grad();
            self.w -= dw * learning_rate; 
        }
        return self.w;
        
    }

    pub fn R_linear(&self, myx: f64) -> f64 {
        // capturando dados
        let mut x = self.x_data.sum_all();
        let mut y = self.y_data.sum_all();
        let mut xy = self.x_data.mul_tesors(&self.y_data).sum_all();
        let mut x2 = self.x_data.elevate_to(2.0).sum_all();
        
        let n = self.x_data.len();
        let media_x = x/n;
        let media_y = y/n;

        let beta = {
            let first = xy - {n * media_x * media_y};
            let second = x2 - {n * media_x.powf(2.0)};
            first / second
        };

        let alpha = media_y - {beta * media_x};
        return alpha + {beta * myx};
    }
}

impl Learn {
    pub fn New(xx: Tensor, yy: Tensor, ww: f64) -> Learn {
        return Learn {x_data: xx, y_data: yy, w: ww};
    }

    pub fn New_From_Vec(xx: Vec<f64>, yy: Vec<f64>, ww: f64) -> Learn {
        let first = Tensor {x: xx};
        let two = Tensor {x: yy};
        return Learn {x_data: first, y_data: two, w: ww};
    }
}

pub struct Tensor {
    x: Vec<f64>
}

// Return Tensors
impl Tensor {


    fn elevate_to(&self, elevate: f64) -> Tensor{
        let f_return: Vec<f64> = vec![];
        for e in self.x.iter() {
            f_return.push(e.powf(elevate))
        }
        return Tensor {x: f_return}
    }

    fn sub(&self, x2: &Vec<f64>) -> Tensor{
        let f_return: Vec<f64> = vec![];
        for (i, e) in self.x.iter().enumerate() {
            f_return.push(e - x2[i]);
        }
        return Tensor {x: f_return};
    }

    fn mul_all_for(&self, num: f64) -> Tensor{
        let f_return = vec![];
        for element in self.x.iter() {
            f_return.push(element*num);
        }
        return Tensor {x: f_return}
    }

    fn multi(&Self, x2: &Vec<f64>) -> Tensor{
        let f_return: Vec<f64> = vec![];
        for (i, e) in self.x.iter().enumerate() {
            f_return.push(e * x2[i]);
        }
        return Tensor {x: f_return};
    }
}

// Return f64 data
impl Tensor {
    fn media(&self) -> f64 {
        return {
            let mut sum = 0.0;
            for element in self.x.iter() {
                sum += element;
            }
            let len = self.x.len() as f64;
            return sum / len;
        }
    }

    fn EMQ(&self) -> f64{
        let media = tensor_media(&self.x);
        let mut penalidade: Vec<f64> = vec![];
        for dn in self.x.iter() {
            let calculo = media - dn;
            if calculo >= 0.0 {
                penalidade.push(calculo);
            }
            else {
                penalidade.push(calculo * -1.0);
            }
        }
        return {
            tensor_sum_all(&penalidade)
        };
    }

    fn sum_all(&self) -> f64 {
        let mut soma = 0.0;
        for elements in self.x.iter() {
            soma += elements;
        }
        return soma;
    }
}

impl Tensor {
    fn new(x) {
        
    }
}
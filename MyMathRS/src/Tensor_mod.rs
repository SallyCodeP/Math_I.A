pub struct Tensor {
    pub x: Vec<f64>
}

// Return Tensors
impl Tensor {

    pub fn mul_tesors(&self, x2: &Tensor) -> Tensor{
        let mut f_return: Vec<f64> = vec![];
        for (i, e) in self.x.iter().enumerate() {
            f_return.push(e * x2.x[i]);
        }
        return Tensor {x: f_return};
    }

    pub fn elevate_to(&self, elevate: f64) -> Tensor{
        let mut f_return: Vec<f64> = vec![];
        for e in self.x.iter() {
            f_return.push(e.powf(elevate))
        }
        return Tensor {x: f_return}
    }

    pub fn sub(&self, x2: &Tensor) -> Tensor{
        let mut f_return: Vec<f64> = vec![];
        for (i, e) in self.x.iter().enumerate() {
            f_return.push(e - x2.x[i]);
        }
        return Tensor {x: f_return};
    }

    pub fn mul_all_for(&self, num: f64) -> Tensor{
        let mut f_return = vec![];
        for element in self.x.iter() {
            f_return.push(element*num);
        }
        return Tensor {x: f_return}
    }

    pub fn len(&self) -> f64 {
        let abc = &self.x;
        return abc.len() as f64;
    }
}

// Return f64 data
impl Tensor {
    pub fn media(&self) -> f64 {
        return {
            let mut sum = 0.0;
            for element in self.x.iter() {
                sum += element;
            }
            let len = self.x.len() as f64;
            sum / len
        };
    }

    pub fn EMQ(&self) -> f64{
        let media = self.media();
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
            let f_return = Tensor{x: penalidade};
            f_return.sum_all()
        };
    }

    pub fn sum_all(&self) -> f64 {
        let mut soma = 0.0;
        for elements in self.x.iter() {
            soma += elements;
        }
        return soma;
    }
}
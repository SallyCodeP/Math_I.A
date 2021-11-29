extern crate cpython;
use cpython::{Python, PyResult};



mod mymath {

    mod Tensor_mod {
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

            fn sum_all(&self) -> f64 {
                let mut soma = 0.0;
                for elements in self.x.iter() {
                    soma += elements;
                }
                return soma;
            }
        }
    }

    mod Learn_mod {
        use crate::mymath::Tensor_mod::Tensor;

        pub struct Learn {
            x_data: Tensor,
            y_data: Tensor,
            w: f64
        }

        impl Learn {

            fn foward_Tensor(&self) -> Tensor {
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

            fn training(&mut self, time: i64)-> f64 {
                let learning_rate = self.get_rate(time);

                for _ in 1..= time {
                    let dw = self.grad();
                    self.w -= dw * learning_rate; 
                }
                return self.w;
                
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
    }

    mod Matrix_mod {
        struct Matrix {
            m1: Vec<Vec<f64>>
        }
    
        impl Matrix {

            fn check_m_len(&self) -> bool {
                let tamanho = self.m1[0].len();
                for element in self.m1.iter() {
                    if tamanho != element.len() || tamanho <= 0 {
                        return false;
                    }
                }
                return true;
            }

            fn multi_matriz(&self, m: &Matrix) -> Result<Matrix, ()> {
                if self.check_m_len() && m.check_m_len() && self.m1[0].len() == m.m1.len() {
                    let mut fin: Vec<Vec<f64>> = vec![];
    
                    
                    let mut colunas_m2: Vec<Vec<f64>> = vec![]; 
                    // Capturando colunas de m2
                        for _ in 0.. m.m1[0].len() {colunas_m2.push(vec![]);}
                        for linha in m.m1.iter() {
                            for (idx, value) in linha.iter().enumerate() {colunas_m2[idx].push(*value);}
                        }
                    
                    let mut capsula: Vec<f64> = vec![];
                        for l in self.m1.iter() {
                            for ele in colunas_m2.iter() {
                                let mut add: Vec<f64> = vec![];
                                for n in 0.. ele.len() {add.push(l[n] * ele[n]);}
                                
                                let mut a = 0.0;
                                for j in add.iter() {
                                    a += j
                                }
                                capsula.push(a)
                            }
                            fin.push(capsula);
                            capsula = vec![];
                        }
                        
                    return Ok(Matrix{m1:fin});
    
                } else {return Err(());}
            }
    

        }
    }

    mod Statistic_mod {
    
        pub fn R_linear(myx: f64, data: Vec<[f64;2]>) -> f64 {
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
    }
}



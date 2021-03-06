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

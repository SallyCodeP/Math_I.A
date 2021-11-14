pub fn multi_matriz(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, ()> {
    if check_m_len(m1) && check_m_len(m2) && m1[0].len() == m2.len() {
        let mut fin: Vec<Vec<f64>> = vec![];

        
        let mut colunas_m2: Vec<Vec<f64>> = vec![]; 
        // Capturando colunas de m2
            for _ in 0.. m2[0].len() {colunas_m2.push(vec![]);}
            for linha in m2 {
                for (idx, value) in linha.iter().enumerate() {colunas_m2[idx].push(*value);}
            }
        
        let mut capsula: Vec<f64> = vec![];
            for l in m1.iter() {
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
            
        return Ok(fin);

    } else {return Err(());}
}

pub fn check_m_len(m1: &Vec<Vec<f64>>) -> bool {
    let tamanho = m1[0].len();
    for element in m1.iter() {
        if tamanho != element.len() || tamanho <= 0 {
            return false;
        }
    }
    return true;
}




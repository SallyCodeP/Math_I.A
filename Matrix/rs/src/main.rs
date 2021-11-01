fn main() {
    println!("Hello, world!");
    let m1 = vec![vec![6.0, 7.0, 2.0], vec![6.0, 5.0, 3.0], vec![1.0, 7.0, 3.0]];
    let m2 = vec![vec![6.0, 8.0, 9.0], vec![6.0, 4.0, 3.0], vec![2.0, 7.0, 1.0]];
    let resposta = match multi_matriz(&m1, &m2) {
        Ok(a) => a,
        Err(_) => {println!("Error"); vec![vec![]]}
    };
    println!("{:?}", resposta);
    
}

fn multi_matriz(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, ()> {
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

fn check_m_len(m1: &Vec<Vec<f64>>) -> bool {
    let tamanho = m1[0].len();
    for element in m1.iter() {
        if tamanho != element.len() || tamanho <= 0 {
            return false;
        }
    }
    return true;
}




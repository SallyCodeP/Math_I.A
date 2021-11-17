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


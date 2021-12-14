/*
extern crate cpython;
use cpython::{Python, PyResult};




fn main() {
    let x: Vec<f64> = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let y: Vec<f64> = vec![4.0, 6.0, 8.0, 10.0, 12.0, 14.0];

    let mut first = Learn_mod::Learn::New_From_Vec(x, y, 1.0);
    first.training(100);
    println!("{}", first.foward_f64(20.0));
}



*/

mod Tensor_mod;
mod Learn_mod;
mod Matrix_mod;


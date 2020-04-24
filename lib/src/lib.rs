#![allow(deprecated)]

use ndarray::*;

#[derive(Debug, Clone, Default)]
pub struct GlobalLpPool {
    p: usize,
}

impl GlobalLpPool {
    pub fn eval(&self, array: Vec<f64>, shape: &[usize]) -> Vec<f64> {
        let array = ndarray::Array::from_vec(array).into_shape(shape).unwrap();
        let n = shape[0];
        let c = shape[1];

        let divisor = array.len() / (n * c);
        let input = array.into_shape(((n * c), divisor)).unwrap();
        let divisor = (divisor as f64).recip();
        let result = if self.p == 1 {
            todo!();
        } else {
            input
                .fold_axis(Axis(1), 0.0, |&a, &b| a + b.abs().powi(self.p as i32))
                .map(|a| a.powf((self.p as f64).recip()) * divisor)
        };
        result.to_vec()
    }
}

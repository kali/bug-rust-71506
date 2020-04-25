#![allow(deprecated)]

use ndarray::*;

#[derive(Debug, Clone, Default)]
pub struct GlobalLpPool {
    p: usize,
}

impl GlobalLpPool {
    pub fn eval(&self, input: Vec<f64>) -> Vec<f64> {
        let input = ndarray::Array::from_vec(input);

        // second call to into_shape() is redundant, but the bug only triggers
        // if it is in there
        let input = input.into_shape((2, 1)).unwrap();
        let input = input.into_shape((2, 1)).unwrap();

        let result = if self.p == 1 {
            unimplemented!("need the if to trigger the bug");
        } else {
            input
                .fold_axis(Axis(0), 0.0, |&a, &b| a + b.powi(self.p as i32))
                .map(|a| a.powf((self.p as f64).recip()))
        };
        result.to_vec()
    }
}

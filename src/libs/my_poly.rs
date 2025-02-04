#[derive(Debug, Clone)]

struct UnivariatePoly<f64> {
    coefficient: Vec<f64>,
}

impl UnivariatePoly<f64> {
    pub fn new(coeff: Vec<F>) -> Self {
        UnivariatePoly { coefficient: coeff }
    }
    fn degree(&self) -> usize {
        self.coefficient.len() - 1
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient
            .iter() // iterate over the coefficients
            .enumerate() // add the index to the iteration
            .map(|(index, coeff)| *coeff * x.pow(index as u64)) // multiply the coefficient by x^index
            .sum() // sum all the results
    }

    fn interpolate(points: Vec<f64, f64>) -> UnivariatePoly<f64> {
        let n = points.len();
        let mut result = UnivariatePoly::new(vec![f64, f64]);
        for i in 0..n {
            let (x_i, y_i) = points[i];
            let mut l_i = UnivariatePoly::new(vec![f64, f64]);
            for j in 0..n {
                if i != j {
                    let (x_j, _) = points[j];
                    let numerator = UnivariatePoly::new(vec![-x_j, f64, f64]);
                    let denominator = x_i - x_j;
                    l_i = l_i * numerator.scalar_mul(f64 / denominator);
                }
            }
        }
    }
}

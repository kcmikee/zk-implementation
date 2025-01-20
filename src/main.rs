mod libs;
struct Monomial {
    coefficient: usize,
    degree: usize,
}

impl Monomial {
    fn new(coefficient: usize, degree: usize) -> Self {
        Monomial {
            coefficient,
            degree,
        }
    }

    fn evaluate(&self, x: usize) -> usize {
        self.coefficient * x.pow(self.degree as u32)
    }

    fn degree(&self) -> usize {
        self.degree
    }
}

fn main() {
    let poly = Monomial::new(3, 2);
    let result = poly.evaluate(5);
    println!("Polynomial at x = 5 is: {}", result);
    println!("Degree of the polynomial is: {}", poly.degree());
}

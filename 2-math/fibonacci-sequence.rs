const ITERS: usize = 44;

fn print_fib(n: usize) {
    let mut x = (1, 1);
    for i in 0..n {
        println!("Term {}: {}", i, x.0);
        x = (x.1, x.0 + x.1)
    }
}

struct Fib {
    x: (usize, usize),
}

impl Fib {

}

impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.1, self.x.0 + self.x.1);
        Some(self.x.0)
    }
}

fn main() {
    print_fib(ITERS);
}
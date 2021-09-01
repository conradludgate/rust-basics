fn main() {
    let mut values = Vec::new();
    for value in FibonaciiIter::new().take(10) {
        values.push(value)
    }
    println!("The first 10 fibonacii values are: {:?}", values);

    let mut values = Vec::new();
    FibonaciiIter::new().skip(10).take(10).for_each(|value| values.push(value));
    println!("The next 10 fibonacii values are: {:?}", values);

    let values: Vec<_> = FibonaciiIter::new().skip(20).take(10).collect();
    println!("The next 10 fibonacii values are: {:?}", values);
}

#[derive(Debug, Clone, Copy)]
struct FibonaciiIter(i32, i32);

impl FibonaciiIter {
    fn new() -> Self {
        Self(0, 1)
    }
}

impl Iterator for FibonaciiIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = *self; // copy the iterator for re-use later

        self.1 += self.0;
        self.0 = temp.1;

        Some(temp.0)
    }
}

fn main() {
    println!("The sixth fibonacii number is {}", fibonacii(6));
}

fn fibonacii(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fibonacii(n - 1) + fibonacii(n - 2);
}

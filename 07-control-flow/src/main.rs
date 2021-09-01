fn main() {
    println!("The sixth fibonacii number is {}", fib_match(6));
    println!("The sixth fibonacii number is {}", fib_for(6));
    println!("The sixth fibonacii number is {}", fib_if_else(6));
}

fn fib_match(n: i32) -> i32 {
    // match is similar to a switch in other languages
    // but it is far more powerful
    match n {
        0..=1 => n,
        _ => fib_match(n - 1) + fib_match(n - 2),
    }
}

fn fib_for(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    // Similar to `for i in range(n):` in python.
    for _ in 0..n {
        let tmp = b;
        b += a;
        a = tmp;
    }

    a
}

fn fib_if_else(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib_if_else(n - 1) + fib_if_else(n - 2)
    }
}

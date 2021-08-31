fn main() {
    println!("The sixth fibonacii number is {}", for_loop::fib(6));
    println!("The sixth fibonacii number is {}", if_else::fib(6));
    println!("The sixth fibonacii number is {}", match_expr::fib(6));
}

// include the module defined at `./for_loop/mod.rs`
mod for_loop;

// include the module defined at `./if_else.rs`
mod if_else;

// include the module as defined here
mod match_expr {
    pub fn fib(n: i32) -> i32 {
        // match is similar to a switch in other languages
        // but it is far more powerful
        match n {
            0..=1 => n,
            _ => fib(n - 1) + fib(n - 2),
        }
    }
}

pub fn fib(n: i32) -> i32 {
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

fn main() {
    // This is a fixed sized array containing 5 elements
    let numbers: [i32; 5] = [0, 1, 2, 3, 4];

    println!("{:?}", numbers);
    //          ^ debug formatting since arrays cannot don't have a 'display' (pretty-print) output

    // This is a array slice
    let numbers_one: &[i32] = &numbers[1..];

    println!("{:?}", numbers_one);

    // Vec (aka vector) is a resizable array
    let mut numbers_five: Vec<i32> = numbers_one.to_vec();
    numbers_five.push(5);

    println!("{:?}", numbers_five);
}

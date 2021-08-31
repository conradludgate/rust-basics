fn main() {
    let x: &str = "world";
    //   ^ specifying the type

    println!("Hello, {}!", x);

    // 'Mut'able (can be modified)
    let mut name = String::from("conrad");
    name.make_ascii_uppercase();

    println!("Hello, {}!", name);
}

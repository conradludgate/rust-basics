fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = "2.0";
    let b = "5.0";

    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;

    let c = divide(a, b)?;

    println!("{}", c);

    match divide(a, 0.0) {
        Ok(value) => println!("got ok {}", value),
        Err(err) => println!("got err {}", err),
    }

    Ok(())
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    let div = a / b;
    if !div.is_finite() {
        Err("DivideByZeroErr")
    } else {
        Ok(div)
    }
}

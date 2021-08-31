use std::{fmt, ops::Div};

fn main() {
    let distance = Measure(5.0, MeterUnit);
    let time = Measure(10.0, SecondUnit);

    println!("distance ({}) / time ({})", distance, time);

    let speed = distance / time;

    println!("\t= speed ({})", speed);
}

#[derive(Debug)]
struct Measure<Number, Unit>(Number, Unit);

// implement a pretty-printing trait for measure,
// assuming that the generic parameters `N` and `T` both can be pretty-printed
// (using the `Display` trait)
impl<N: fmt::Display, T: fmt::Display> fmt::Display for Measure<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug)]
struct MeterUnit;
#[derive(Debug)]
struct SecondUnit;

impl fmt::Display for MeterUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "m")
    }
}
impl fmt::Display for SecondUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s")
    }
}

#[derive(Debug)]
struct MetersPerSecondUnit;

// implement a 'divide' trait for MeterUnit / SecondUnit
impl Div<SecondUnit> for MeterUnit {
    type Output = MetersPerSecondUnit;

    fn div(self, _: SecondUnit) -> Self::Output {
        MetersPerSecondUnit
    }
}

// implement a 'divide' trait for Measure<N, T> / Measure<M, U> only when N can be divided by M and T can be divided by U
impl<N, M, T, U> Div<Measure<M, U>> for Measure<N, T> where N: Div<M>, T: Div<U> {
    type Output = Measure<N::Output, T::Output>;

    fn div(self, rhs: Measure<M, U>) -> Self::Output {
        Measure(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl fmt::Display for MetersPerSecondUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "m/s")
    }
}

fn main() {
    let foo = NormalStruct {
        some: 1,
        data: "foobar".into(),
        flag: true,
    };

    let bar = TupleStruct(2, false);

    let baz = UnitStruct;

    let five_meters = Metersf64::new(5.0);
    let ten_seconds: Secondsf64 = Measure(10.0, SecondUnit);

    println!("{:?}", foo);
    println!("{:?}", bar);
    println!("{:?}", baz);
    println!("{:?}", five_meters);
    println!("{:?}", ten_seconds);
}

// This is a standard struct, which has named struct fields
#[derive(Debug)]
struct NormalStruct {
    some: u32,
    data: String,
    flag: bool,
}

// This is a 'tuple struct', which has unnamed struct fields
#[derive(Debug)]
struct TupleStruct(u32, bool);

// A unit struct which has no fields.
#[derive(Debug)]
struct UnitStruct;

// This is a tuple struct that features generics
// notice that we define a set of type parameters in `< ... >`
#[derive(Debug)]
struct Measure<Number, Unit>(Number, Unit);

// Example of how unit structs could be used
#[derive(Debug)]
struct MeterUnit;
#[derive(Debug)]
struct SecondUnit;

// type aliases
type Meters<Number> = Measure<Number, MeterUnit>;
type Seconds<Number> = Measure<Number, SecondUnit>;

type Metersf64 = Meters<f64>;
type Secondsf64 = Seconds<f64>;


impl<N> Meters<N> {
    pub fn new(n: N) -> Self {
        Self(n, MeterUnit)
    }
}

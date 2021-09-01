// In rust, data can only have a single 'Owner', and a single 'Mutator' at any one time.
//

fn main() {
    // `hello` is the owner of this String.
    let hello = String::from("Hello, World!");

    println!("hello: {}", hello);

    // Take a reference to `hello`, also known as 'borrowing'
    let len = length(&hello);

    println!("length: {}", len);

    // Give ownership of the string from `hello` to `greeting`.
    // `hello` no longer exists in this scope
    let mut greeting = hello;

    // println!("compile error: {}", hello);
    println!("greeting: {}", greeting);

    // Take a mutable reference to `greeting`, also known as 'mutable borrowing'.
    // This allows another function to modify a local value
    // without 'taking ownership'
    make_uppercase(&mut greeting);

    println!("greeting: {}", greeting);

    let bytes = into_bytes(greeting);

    // println!("compile error: {}", greeting);
    println!("bytes: {:?}", bytes);
}

// `&` prefix. This accepts a reference
// Doesn't take ownership, has read-only access
fn length(s: &str) -> usize {
    s.len()
}

// `&mut` prefix, this accepts a mutable reference
// Doesn't take ownership but has exclusive rights to update the data
fn make_uppercase(s: &mut str) {
    s.make_ascii_uppercase()
}

// No `&` prefix on the type. Takes ownership of the data given to it
fn into_bytes(s: String) -> Vec<u8> {
    s.into_bytes()
}

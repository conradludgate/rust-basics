fn main() {
    // This is a 'string reference', also known as a 'string slice'.
    // It cannot be resized
    // It's typed `&str` where `&` means reference
    // and `str` is our string
    let greeting: &str = "Hello";

    // This is an `owned string`.
    // It can be resized, but can also be turned into a string slice easily
    let mut message: String = String::from(greeting);

    message.push_str(", world!");

    println!("{}", message);

    // this is the 'slice' part
    let name: &str = &message[7..12];
    //               ^       ^^^^^^^ this is the slice part of `string slice`.
    //               |               It's a slice (portion) of a string
    //               \
    //                 This is the reference part of `string reference`.
    //                 The data we want is not the 5 bytes directly, but just a reference to them
    //
    // See https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices for more detail

    println!("Goodbye, {}!", name);
}

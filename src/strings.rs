// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data strucutre - Use when you ned to modify or own string data


pub fn run () {
    let hello = String::from("Hello");
    println!("{}", hello);
    }
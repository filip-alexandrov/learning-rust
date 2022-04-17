// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure -
// Use when you need to modify or own string data

pub fn run() {
    // immutable, fixed length
    let hello = "Hello";

    let mut greeting = String::from("Hello ");

    // Get length
    println!("Length: {}", greeting.len());

    // Only a char
    greeting.push('W');

    // Push another string
    greeting.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", greeting.capacity());

    // Check if is empty
    println!("Is empty: {}", greeting.is_empty());

    // Contains
    println!("Contains 'World' {}", greeting.contains("World"));

    // Replace
    println!("Replace: {}", greeting.replace("World", "There"));

    // Loop through string by whitespace
    for word in greeting.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}

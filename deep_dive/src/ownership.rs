pub fn run() {
    // Rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        // s is not valid because is not declared yet
        let s = "hello"; // fix size on the stack

        // auto memory allocation on the heap
        let str = String::from("hello");
    } // s / str are dropped

    let x = 5;
    let y = x; // copy, simple types are stored on the stack and copied by default, int, bool, float...

    let s1 = String::from("hello"); // stack points to heap (ptr, len, capacity)
    let s2 = s1; // move s1 to s2, heap copy expensive => s1 is dropped

    let s3 = s2.clone(); // deep copy in heap, expensive
    takes_ownership(s3); // s3 is moved

    takes_ownership(x.to_string()); // x is simple and copied, still can be used

    let s4 = String::from("hello");
    let len = calculate_length(&s4); // s4 is borrowed (passing references)

    let mut s5 = String::from("hello");
    change(&mut s5); // mutable reference, only one mutable reference, prevent data races

    // immutable references cannot be mixed with mutable references
    let r1 = &s5;
    let r2 = &s5;

    println!("{} {}", r1, r2);

    // r1 / r2 are out of scope => new mutable reference is possible
    let r3 = &mut s5;

    // Rules:
    // 1. You can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    // Slices
    let s6 = String::from("hello world");
    let hello = &s6[..5]; // reference to part of the string
    let world = &s6[6..11]; // &s6[6..]
                            // s6.clear(); -> need mut reference, but only immutable references are allowed

    let word = first_word(&s6);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // &[2, 3]
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string (s3) is dropped

// references
fn calculate_length(s: &String) -> usize {
    // pointer chain:  s -> s1 -> heap
    // s is immutable
    let length = (*s).len();
    length
} // s is dropped

fn change(some_string: &mut String) {
    // mutates without taking ownership
    some_string.push_str(" world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

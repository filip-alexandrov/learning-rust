pub fn run() {
    let x = 5;
    // x = 6; // let makes x immutable

    let mut x1 = 5;
    x1 = 6;

    // must be type annotated, no runtime value assigned
    const MAX_POINTS: u32 = 100_000;

    // Shadowing
    let x = 10;

    // Scalar datatypes
    // int: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    // float: f32, f64
    // bool: true, false
    // char: 'a'

    // Compound datatypes
    // tuple (fixed len, different types)
    let up = ("up", 1);

    // destructuring
    let (channel, subcount) = up;
    let sub_count = up.1; // value at index 1

    // Arrays, fixed length
    let error_codes = [100, 4004, 500];
    let first_error_code = error_codes[0];

    let byte = [0; 8]; // 8 values all 0

    let number = 10;

    if number < 5 {
        println!("condition was true");
    } else if number < 10 {
        println!("condition was false");
    } else {
        println!("condition was false");
    }

    let number = if true { 5 } else { 6 };

    // infinite loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("again!");
        break counter;
    };

    while counter != 10 {
        println!("{}!", number);
        counter += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }
}

fn my_function() {
    println!("another function")
}

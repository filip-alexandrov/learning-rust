enum IpAddrKind {
    V4(String),
    V6(u8, u8, u8, u8),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn some_function() {
        println!("some function");
    }
}

/* enum Option<T> {
    Some(T),
    None,
} */

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

pub fn run() {
    // variants of ipaddrkind type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

    // optional values
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    // extract value (handle all cases)
    let x = 5;
    let y = Some(5);

    // default 0 value
    let sum = x + y.unwrap_or(0);

    value_in_cents(Coin::Quarter(UsState::Alaska));

    plus_one(y);

    // with if let we can specify only the pattern we care about
    if let Some(3) = some_number {
        println!("three");
    }
}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // binds to state value
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // return Some wrapper because return value is optional
    match x {
        None => None,
        // match exact value
        Some(3) => Some(3),
        Some(i) => Some(i + 1),
        // if its any other pattern
        _ => None,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn run() {
    let mut user1 = User {
        email: String::from("email@email.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;

    user1.username = String::from("new_username");

    let user2 = build_user(String::from("mail@mail.com"), String::from("username"));

    let user3 = User {
        email: String::from("mail@mailcom"),
        username: String::from("username"),
        ..user1 // copy the rest of the fields from user1
    };

    // Tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // automatic referencing and dereferencings
    rect.area();

    println!("{:#?}", &rect);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    rect.can_hold(&rect1);

    // associated function usable directly
    let sq = Rectangle::square(3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// house functions and methods associated with a structs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// multiple impl blocks possible
impl Rectangle {
    // associated function without self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

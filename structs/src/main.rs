// a struct definition
// with 4 fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

// derive the Debug trait automagically
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Box {
    width: u32,
    height: u32,
    depth: u32,
}

impl Box {
    // associated functions
    // &self: shorthand for self: &Self
    // Self being the impl block data type
    fn volume(&self) -> u32 {
        self.width * self.height * self.depth
    }

    
}

// multiple impl blocks for one type are allowed
impl Box {
    // don't need &self
    fn pack_it(size: u32) -> Box {
        Box {
            width: size,
            height: size,
            depth: size,
        }
    }
}

fn main() {
    // instance of the user struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // struct update syntax
    // new user with values from old one
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        // i love this especially
        width: dbg!(30 * 3),
        height: 50,
    };

    // {:?} signals a debug print
    println!("rect1 is {:?}", rect1);

    // {:#?} signals pretty debug print
    println!("rect1 is {:#?}", rect1);

    // dbg! macro: takes ownership, prints file and line number
    dbg!(&rect1);

    let highbox = Box {
        width: 10000,
        height: 2000,
        depth: 3,
    };

    println!("Volume of highbox is: {}", highbox.volume());

    let squarebox = Box::pack_it(19);
}

// function showing of the field init shorthand syntax
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
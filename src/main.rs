fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("John Smith"),
        email: String::from("john@mail.com"),
        sign_in_count: 12,
    };

    println!("User1 name is: {}", user1.username);
    user1.username = String::from("Kate");
    println!("User1 mutated name is: {}", user1.username);

    let user2 = build_user(String::from("user2"), String::from("user2@mail.com"));
    println!("User2 is: {} with email: {}", user2.username, user2.email);

    let black = Color(0, 0, 0);
    let start_point = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let dimension = (10, 14);
    let area_one = area_tuple(dimension);
    println!("Area one is: {}", area_one);
    println!("Dimension is {} width and {} height", dimension.0, dimension.1);

    let rect = Rectangle {
        width: 10,
        height: 3,
    };

    println!("New are is: {} pixels", area_struct(&rect));
    println!("Rectangle info: {rect:?}");
    println!("Rectangle info: {rect:#?}");
    dbg!(&rect);

    println!("Area impl is: {}", rect.area());
    let rect2 = Rectangle {
        width: 7,
        height: 4,
    };
    println!("Can hold: {}", rect.can_hold(&rect2));

    let square = Rectangle::square(14);
    println!("Square is: {square:#?}")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

   
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
 
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct AlwaysEqual;

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

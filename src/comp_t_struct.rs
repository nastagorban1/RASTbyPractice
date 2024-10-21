// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn first() {
    let age = 30;
    let hobby =  String::from("dancing");
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby
    };

    println!("Success!");
}


// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn second() {
    let v = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Point) {
    let Point (x, y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }


// Fill the blank and fix the error without adding/removing new line
struct Person1 {
    name: String,
    age: u8,
}
fn third() {
    let  age = 18;
    let mut p = Person1 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}


struct Person2 {
    name: String,
    age: u8,
}
fn four() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person2 {
    Person2 {
        age,
        name,
    }
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn five() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}




#[derive(Debug)]    //макрос
struct Rectangle {
    width: u32,
    height: u32,
}

fn six() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}
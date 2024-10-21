pub(crate) fn first() {

    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);


}

// Don't modify code in main!
pub(crate) fn second() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}



pub(crate) fn third() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}

// Fix the error without removing any code
pub(crate) fn four() {
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}

// Don't use clone ,use copy instead
pub(crate) fn five() {
    let x:(i32,i32,(),&str) = (1, 2, (), "hello");
    let y:(i32,i32,(),&str) = x;
    println!("{:?}, {:?}", x, y);
}


// make the necessary variable mutable
pub(crate) fn six() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}


pub(crate) fn seven() {
    let x = Box::new(5);

    let mut y:Box<i32> =  Box::new(1);     // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}
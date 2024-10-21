fn first() {
   let x = 5;
   // Fill the blank
   let p = &x;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

fn second() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

// Fix error
fn third() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}




// Fix error
fn four() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}


fn five() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = & mut s;

    p.push_str("world");

    println!("Success!");
}


fn six() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}


fn seven() {
    let mut s = String::from("hello");

    let r1 = & s;
    let r2 = & s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}
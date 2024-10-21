#[test]
fn first() {
    let too_long_tuple  = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}


#[test]
fn second() {
    let mut tup:(i32,f64,&str) = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let x = tup.0;
        let y = tup.2;
            let z = tup.1;



    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}







#[test]
fn third() {
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}


#[test]
fn four() {
    // Fill the blank, need a few computations here.
    let nums:(i32,i32) = (2,3);
    let (x, y) = sum_multiply(nums);

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
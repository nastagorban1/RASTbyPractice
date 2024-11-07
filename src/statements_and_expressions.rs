 // Make it work with two ways
fn first() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
    println!("Success!");
}




    fn second () {
        let v = {
            let x = 3;
            x
        }
            ;

        assert!(v == 3);

        println!("Success!");
    }

    fn third () {
        let s = sum(1, 2);
        assert_eq!(s, 3);

        println!("Success!");
    }

fn sum(x: i32, y: i32) ->i32 {
    x + y}


// Fill in the blank
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a  {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}

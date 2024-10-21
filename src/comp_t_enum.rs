 // Fix the errors
enum Number {
    Zero,     //0
    One,         //1
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn first() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One   as u8, Number2::One   as u8);

    println!("Success!");
}


 // Fill in the blank
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn second() {
    let msg1 = Message::Move{x:1,y:2}; // Instantiating with x = 1, y = 2
    let msg2 = Message::Write("hello,world!".to_string()); // Instantiating with "hello, world!"

    println!("Success!");
}

 enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn third() {
    let msg = Message1::Move{x: 1, y: 1};

    if let Message1::Move{x:a,y:b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}



 #[derive(Debug)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn four() {
    let msgs: [Message2;3] = [
        Message2::Quit,
        Message2::Move{x:1, y:3},
        Message2::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message2) {
    println!("{:?}", msg);
}

fn five() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(six);

     if let Some(n)   = six {
        println!("{}", n);

        println!("Success!");
    } else {
     panic!("NEVER LET THIS RUN！");
    }



}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
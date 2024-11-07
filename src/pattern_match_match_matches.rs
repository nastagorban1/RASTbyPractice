fn first() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean{
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}





fn second() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main() {
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0)
        ];

        for msg in msgs {
            show_message(msg)
        }

        println!("Success!");
    }

    fn third(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => { // match  Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(r, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            __ => println!("no data in these variants")
        }
    }
}


fn four() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    if let Some(i)= o {

            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }

}



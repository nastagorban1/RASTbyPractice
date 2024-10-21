
      fn first (){
    let c1 = '中';
    print_char(c1);
}

fn print_char(c : char) {
    println!("{}", c);
}

//Make println! work
   fn second () {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

   // Make it work
   fn third () {
       let f = true;
       let t = true || false;
       assert_eq!(t, f);

       println!("Success!");
   }

//Make it work, don't modify `implicitly_ret_unit` !
   fn four (){
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());   //0 bites

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


   // Modify `4` in assert to make it work
use std::mem::size_of_val;
   fn five () {
       let unit: () = ();
       assert!(size_of_val(&unit) == 0);

       println!("Success!");
   }
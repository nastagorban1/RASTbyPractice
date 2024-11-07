
fn first() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Це метод, який використовує self, щоб мати доступ до полів структури
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    fn main() {
        let rect = Rectangle { width: 30, height: 50 };
        println!("Площа прямокутника: {}", rect.area()); // Виклик методу через екземпляр
    }
}


fn second() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Перший impl блок для методу `area`
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // Другий impl блок для методу `can_hold`
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 20 };

        println!("Площа rect1: {}", rect1.area());
        println!("rect1 може вмістити rect2: {}", rect1.can_hold(&rect2));
        println!("Success!");
    }
}
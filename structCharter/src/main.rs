use std::io;


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    let mut rect1 = Rectangle {
        width: 0,
        height: 0,
    };
    loop {
        println!("Please input your width then height");
        let mut width = String::new();
        io::stdin()
            .read_line(&mut width)
            .expect("Failed to read line");

        let width: u32 = match width.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("FUCK.");
                continue
            },
        };
        rect1.width = width;

        let mut height = String::new();
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read line");

        let height: u32 = match height.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("FUCK.");
                continue
            },
        };
        rect1.height = height;

         println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
}


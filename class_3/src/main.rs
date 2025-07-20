// STRUCTS 
// in strings -> i cant use copy trait

// #[derive(Clone)]
// struct User {
//     active: bool,
//     sign_in_count: u64,
//     username: String,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         sign_in_count: 1,
//         username: String::from("Pihu")
//     };

//     print_name(user1.clone());
//     println!("user is active: {}", user1.active);
// }

// fn print_name(user1: User) {
//     print!("{:?}", user1.active);
// }




// IMPLEMENTS IN STRUCTS

// struct Rect {
//     width: u32,
//     height: u32
// }

// impl Rect {
//     fn area(self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 30
//     };

//     print!("area of rectangle is {}", rect.area());
// }




// ENUMS

use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}


fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(5.0, 6.0);

    println!("area of circle is {}", calculate_area(circle));
    println!("area of square is {}", calculate_area(square));
    println!("area of rectangle is {}", calculate_area(rectangle));         
}


fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}




// ERROR-HANDLING

// use std::fs;

// fn main() {
//     let greeting_result = fs::read_to_string("hello.txt");

//     match greeting_result {
//         Ok(file_content) => {
//             println!("file read: {:?}", file_content);
//         },
//         Err(error) => {
//             println!("failed to read file: {:?}", error);
//         }
//     }
// }
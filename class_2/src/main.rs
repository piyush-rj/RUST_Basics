struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    let user1 = User {
        active: true,
        username: String::from("Piyush"),
        email: String::from("adsp@gmail.com"),
        sign_in_count: 2
    };

    println!("{:?}", user1.username);


    // BORROWING AND OWNERSHIP 

    // let mut st = String::from("hello");
    // let ref1 = &mut st;
    // let final_str = add_world(ref1); // returns a copy

    // println!("{}", final_str);

}

// fn add_world(s: &mut String) -> String {
//     s.push_str(" world");
//     s.clone()
// }



// mut -> making a variable mutable
// &mut -> borrowing a variable to likely change it
// String -> making a variable the owner
// &String -> borrowing a variable to read
















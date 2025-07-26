use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct User {
    username: String,
    password: String
}

fn main() {
    let u = User {
        username: String::from("piyuh"),
        password: String::from("123")
    };

    let mut v: Vec<u8> = Vec::new();
    let ans = u.serialize(&mut v);
    let user = User::try_from_slice(&v).unwrap();
    println!("{}", user.username);
}


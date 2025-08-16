// use serde::{Serialize, Deserialize};


// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     username: String,
//     password: String
// }

// fn main() {

//     let u = User {
//         username: String::from("piyush"),
//         password: String::from("123")
//     };

//     let serialized_string = serde_json::to_string(&u)
//         .expect("error while serializing");
//     println!("{}", serialized_string);

//     let deserialized_string: User = serde_json::from_str(&serialized_string)
//         .expect("error while deserializing");
//     println!("{:?}", deserialized_string);

// }


use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    username: String,
    // password: String
}

fn main() {
    let u = User {
        username: String::from("Piyush"),
        // password: String::from("124")
    };

    let mut v: Vec<u8> = Vec::new();
    let serialized_user = u.serialize(&mut v);

    println!("{:?}", v);

    let deserialized_user = User::try_from_slice(&mut v);

    println!("{:?}", deserialized_user);
}
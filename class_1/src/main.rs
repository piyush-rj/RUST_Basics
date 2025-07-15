fn main() {
    let name: String = String::from("Piyush");
    let (len, name) = get_len(name);

    println!("{}", len);
    println!("{}", name);
}


fn get_len(s: String) -> (usize, String) {
    return (s.len(), s);
}

// here the ownership of name is being transferred to len, hence printing/ calling name after defining ownership of len throws error.

fn main() {
    println!("hello");
    let ans = increment(1, 1);
    println!("{}", ans);
}

fn increment(count: u32, val: u8) -> u32 {
        let val = count + 1;
        return val;
}
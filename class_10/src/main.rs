// lifetimes

fn main() {
    let str = String::from("anjan");
    let mut ans;

    {
        let str2 = String::from("anjan suman");
        ans = access_string(&str, &str2);
        println!("{}", str2);
        println!("{}", ans);
    };
}


fn access_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } 
    return s2;
} 
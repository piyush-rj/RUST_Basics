mod array_list;
use array_list::ArrayList;

fn main() {
    let mut list = ArrayList::init();
    list.push("anjan");
    list.push("gunnu bhaiya");
    list.push("didi");
    list.push("piyush");

    println!("element at 1st index is {:?}", list.get(1));

    let s = list.size();
    println!("size is {}", s);

    list.print();
}

// T -> any type
// E -> error type
// K, V -> key, value types


trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    height: u32,
    width: u32
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn get_area(s: impl Shape) -> u32 {
    return s.area();
}


fn main() {
    let u = Rect {
        width: 10,
        height: 20
    };

    print!("{}", get_area(u));
}
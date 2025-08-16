use std::f32::consts::PI;

trait Shape {
    fn area(&self) -> f32;
}

struct Rect {
    width: f32,
    height: f32
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.height * self.width;
    }
}

struct Circle {
    radius: f32
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI * self.radius * self.radius;
    }
}

fn print_area_of_shape<T: Shape>(c: T) {
    println!("area is {}", c.area());
}

fn main() {
    let r = Rect {
        width: 10.0,
        height: 10.0
    };

    let c = Circle {
        radius: 2.0,
    };

    println!("{}", c.area());
    println!("{}", r.area());

    print_area_of_shape(c);
}
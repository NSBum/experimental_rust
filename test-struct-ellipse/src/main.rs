#[derive(Debug)]
struct Ellipse {
    major: u32,
    minor: u32
}

impl Ellipse {
    fn area(&self) -> f32 {
        return 3.14159 * self.major as f32 * self.minor as f32;
    }
}


fn main() {
    let e = Ellipse {
        major: 3,
        minor: 2
    };
    let a = e.area();
    println!("{:?}", a);
}

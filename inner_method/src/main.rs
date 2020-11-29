#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn inner(&mut self) {
        self.x += 1.0;
        self.y += 1.0;
    }
    fn inner_wraper(&mut self) {
        self.inner()
    }
}

fn main() {
    let mut point = Point { x: 3.0, y: 4.0 };
    println!("{:?}", point);
    point.inner_wraper();
    println!("{:?}", point);
}

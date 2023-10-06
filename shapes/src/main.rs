struct Square {
    side: f32,
}
struct Circle {
    radius: f32,
}
struct Rectangle {
    height : f32,
    width : f32,
}
trait Shape {
    fn area(&self) -> f32;
    fn surrounding(&self) -> f32;
}
impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
    fn surrounding(&self) -> f32 {
        self.side * 4.0
    }
}
impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
    fn surrounding(&self) -> f32 {
        self.radius * 2.0 * std::f32::consts::PI
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.height * self.width
    }
    fn surrounding(&self) -> f32 {
        (self.height + self.width) * 2.0
    }
}
fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Square { side: 5.0 }),
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { height: 4.0, width: 3.0 }),
    ];
    for shape in shapes.iter() {   
        println!("Aire: {:?} m^2     Perimètre: {:?} m", shape.area(), shape.surrounding()); 
    }
}

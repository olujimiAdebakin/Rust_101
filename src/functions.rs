#![allow(unused)]



#[derive(Debug)]

struct Point{
     x: f32,
     y: f32,
}
// Struct methods
impl Point {
    
    // Associated function
    // operates on the type itself and not on the instance    
    fn Zero() -> Self {
        Self {x: 0.0,
            y: 0.0}
    }

    // methods
    // operates on the instance of a type and creates a new instance of the type

    fn move_to (&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }


}

fn main() {
    let mut p = Point::Zero();
    println!("{:?}", p);
    p.move_to(1.0, 2.0);
    println!("{:?}", p);

    let d = p.distance_from_origin();
    println!("{}", d);
}

// #![allow(unused)]

// // Struct

// #[derive(Debug)]
// struct Point {
//     x: f32,
//     y: f32,
// }

// // canonical tuple struct
// struct Point3d(f32, f32, f32);

// // empty struct
// struct Empty;

// #[derive(Debug)]
// struct Circle {
//     center: Point,
//     radius: u32,
// }

// fn main() {
//     // Create
//     let p = Point { x: 1.0, y: 2.0 };
//     println!("point.x = {}, point.y = {}", p.x, p.y);

//     let p = Point3d(1.0, 2.0, 3.0);
//     println!("point 3d, {}, {}, {}", p.0, p.1, p.2);

//     let empty = Empty;

//     let circle = Circle {
//         center: Point { x: 1.0, y: 2.0 },
//         radius: 10,
//     };
//     // Debug
//     // Read
//     println!("circle = {:#?}", circle);

//     // shortcut
//     let x = 1.0;
//     let y = 2.0;
//     let point = Point { x, y};
//     println!("point = {:#?}", point);

//     // copy field
//     let p0 = Point {x : 1.0, y: 1.0, z: 1.0};
//     let p1 = Point {x: 2.0, ..p0};
//     println!("{:#?}", p1);

//     // Update
//     let mut p = Point {x: 0.0, y: 0.0};
//     p.x += 1.0;
//     p.y += 2.0;
//     println!("{:#?}", p);
// }

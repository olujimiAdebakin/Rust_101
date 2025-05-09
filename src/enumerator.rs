#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, u8),
    Hex(String),
    Hsl {h: u8, s: u8, l: u8},
}
fn main(){

    // Enum are custom data that we can declare to represent finite states or values
    // Enum can be used to represent the different states of a variable
    // Enum

    let color: Color = Color::Red;
    let color: Color = Color::Green;
    let color: Color = Color::Blue;
    let color: Color = Color::Rgba(215, 200, 100, 111);
    let color: Color = Color::Hex("FF0000".to_string());
    let color: Color = Color::Hsl {h: 0, s: 0, l: 0};
    // Attribute debug - partialEq

    // Debug
    println!("{:?}", color);

    // partialEq
    // This uses the PartialEq trait to compare values for equality.
    // PartialEq allows the == and != operators.

// This is only possible if the Color enum (or struct) derives or implements Partial
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    // option
    let x: Option<i32> = None;
    let x: Option<i32> = Some(132);
    println!("option: {:?}", x);

    // result = Ok(5) | Err("div by 0")
    let res: Result<u32, &str> = Ok(5);
    let res: Result<u32, &str> = Err("div by 0");
    println!("result: {:?}", res);
    

}
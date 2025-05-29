#![allow(unused)]
// cargo fmt
// fn main() {
//     // let x: i32 = 132;
//     let mut x: i32 = 45;
//     x += 2;

//      println!("The value of x is: {}", x);
// }
mod enumerator; 
mod structure;
mod operators;
mod loops;

use core::net;

#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "Rust";
    println!("Hello, {}", lang);
    println!("Hello, {} {}", lang, lang);

    println!("Hello, {lang}");

    let x = 3;
    println!("{0} x {0} = {1}", x, x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.0".to_string(),
    };

    println!("{:?}", lang);
    println!("{:#?}", lang);

    vary();
    scale();
    compound_types();
    string();
    methods();
    operates();
    elseif();
    loops();
}

// variables in rust

fn vary() {
    let x: i32 = -132;
    // this will not compile
    // x += 1;

    let mut y: i32 = 123;
    y += 2;
    println!("{}", y);

    let z = -123;
    // let w: () = 123;

    // type placeholder
    let x: _ = 5;

    let v: Vec<_> = vec![1, 2, 3];
}

// scalar types

fn scale() {
    // Scalar types
    // -single value
    //  - building blocks for more complex types
    // Integers
    // -2(**(n-1)) - 2**(n-1) - 1
    let i0: i8 = -128; // - 2**(8-1) - 2**(8 - 1) - 1
    //  - 128   -127
    let i1: i16 = 1;
    let i2: i32 = 1;
    let i3: i64 = 1;
    let i4: i128 = 1;
    let i5: isize = 1;

    // Signed integers
    // unsigned integers

    let u0: u8 = 1; // 0 -2**8 - 1 = 255
    let u1: u16 = 1;
    let u2: u32 = 1;
    let u3: u64 = 1;
    let u4: u128 = 1;
    let u5: usize = 1;

    // floats

    let f0: f32 = 0.01;
    let f1: f64 = 0.01;

    // boolean

    let b: bool = true;
    let c: bool = false;
    // characters

    let c: char = 'c';
    let e: char = 'f';

    // Type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and Max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("char min: {min_char}");
    println!("char max: {max_char}");

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 1;
    println!("overflow u32: {u}");

    // checked add - Some(x) | None
    let mut u = u32::checked_add(u32::MAX, 1);
    println!("checked add: {:?}", u);

    // wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", u);
}

fn compound_types() {
    // compound data types
    // - tuple
    // - array
    // - struct
    // - enum

    // Tuple
    let t: (bool, u32, char) = (true, 1, 'c');

    // Destructure
    let (a, b, c) = t;

    // ignore with -
    let (_, b, c) = t;

    // Empty tuple - unit type
    let t = ();

    // Nested tuple
    let nested = ((1.23, 'a'), (true, 1u32, "told"), ());
    println!("nested {} {} {}", nested.0.0, nested.0.1, nested.1.1);
    // accessing the tuple
    // u access a tuple by index
    let t: (bool, u32, char) = (true, 1, 'c');
    println!("t = {}, {}, {}", t.0, t.1, t.2);

    // Array
    // array - fixed length, Known at compile time
    let arr: [u32; 4] = [1, 2, 3, 4];
    println!("arr[2] = {}", arr[2]);

    // update an element in an array
    let mut arr: [u32; 5] = [1, 2, 3, 4, 5];
    arr[2] = 11;
    println!("arr[] = {:?}", arr);

    // declare an array[] and later fill it
    let arr: [u32; 10] = [0; 10];
    println!("{:?}", arr);

    // slice - length not known at compile time
    let num: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
    println!("num = {:?} ", num);

    // get first 3 elements
    let s = &num[0..6]; // 0, 1,
    println!("first 3 elements: {:?} ", s);

    // middle 4 elements
    let s = &num[3..7];
    println!("middle 4 elements: {:?}", s);

    // get the last 3 elements
    let s = &num[7..10];
    println!("last 3 element: {:?}", s);

    // get all elements
    let s = &num[..];
    println!("all elements: {:?} ", s);
}

fn string() {
    // String = vector of u8 (Vec<u8>) valid UTF-8
    // &str = slice of u8 (str) valid UTF-8

    // when to use String vs &str
    // String -> mutate or data needs to be owned ie immutable
    // &str -> data does not need to be owned ie read only

    // String
    let msg: String = String::from("Hello Rust");
    let len: usize = msg.len();
    println!("msg: {msg}");
    println!("len: {len}");

    let mut msg = String::from("Hello");
    msg.push_str(" Rust"); // ✅ This works!
    println!("{}", msg); // Hello Rust

    // str - string slice
    // &str
    // - usually used str with reference(borrowed)
    // - immutable

    let msg: String = String::from("Hello Rust");
    let s: &str = &msg[0..5];
    let len: usize = s.len();
    println!("slice: {s}");
    println!("len: {len}");

    // declaring a variable with a string literal
    // String literal
    // slice pointing to a specific part of the binary
    // immutable because hard-coded inside binary
    // Deref coercion

    let s: &str = "Hello";

    let s: &str = "hi";
    println!("{:?}", s);

    let s: &str = r#"
    {
        "name": "John",
        "age": 30,
        "city": "New York",
        "b": {"c": 3}
    }
    "#;
    println!("{:#}", s);

    // Deref coercion
    // Deref coercion is Rust's way of converting a reference like &String into a &str (or other target types) automatically by calling deref() behind the scenes. This makes code cleaner and more intuitive.
    let msg: String = String::from("Hello Rust");
    let s: &str = &msg;
    println!("{}", s);

    // Add &str to String
    let mut msg: String = "Hello".to_string();
    msg += "!";
    println!("{}", msg);

    // sting interpolation
    let lang = "Rust";
    let emoji = "🦀";
    let msg = format!("Hello {lang} {emoji}");
    println!("{}", msg);

}

#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
}
// Struct methods
impl Point {
   
   // Associated function
   // operates on the type itself and not on the instance    
   fn zero() -> Self {
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

fn methods() {
   let mut p = Point::zero();
   println!("{:?}", p);
   p.move_to(1.0, 2.0);
   println!("{:?}", p);

   let d = p.distance_from_origin();
   println!("distance: {}", d);
}


fn operates() {
    
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32 = a * b;
    println!("{}", c);

    let a: f32 = 1.0;
    let b: f32 = 2.0;
    let c: f32 = a + b;
    println!("{}", c);

    // literals 
    let a  = 1i32;
    let b = 2u64;
    let c = a + b as i32;

    // Boolean
    let a  = true && false;
    let a  = true || false;
    let a  = !true;

    // Bitwise
    // 101
    let a:  u8 = 5;
    // 011
    let b: u8 = 3;
    println!("a & b = {:03b}", a & b);
    println!("a | b = {:03b}", a | b);
    println!("a ^ b = {:03b}", a ^ b);
    println!("!a = {:03b}", !a);
    println!("1 << 3 = {:03b}", 1 << 3);
    println!("10 >> 2 = {}", 10u32 >> 2);
}


fn elseif(){
    let a = 4;
    if a > 0 {
        println!("a is positive");
    } else if a < 0 {
        println!("a is negative");
    } else {
        println!("a is zero");
    }

    println!("a = {a}")
}


fn loops(){

    // loop
    // the loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tel it to stop.

    let mut i = 0;
    loop{
        println!("loop");
        if i == 10 {
            break;
        }
        i += 2;
    }

    // while loop
    // works only whn the condition is true
    let mut i = 0;

    while i <= 3 {
        println!("while {i}");
        i += 1; 
    }

    
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
    
        while index < 5 {
            println!("the value is: {}", a[index]);
    
            index += 1;
        }
   
    // for loop
    for i in 0..= 4 {
        println!("for loop {i}");
    }

    // for loop array
    let a  = [10, 20, 30, 40, 50];
    for i in a {
        println!("for loop array {i}");
    }

    // usize and range
    let n: usize = a.len();
    for i in 0..n {
        println!("for loop array {i}");
    }

    // for loop vector

    let v = vec![10, 20, 30, 40, 50];
    for i in v {
        println!("for loop vector {i}");
    }

    let v = vec![10, 20, 30, 40, 50];

      // iter 
    // aloows multiple loops 
    for i in v.iter(){
        println!("for loop vector {i}");
    }


    for i in v.iter() {
        println!("for loop vector {i}");
    }

}
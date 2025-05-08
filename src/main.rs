#![allow(unused)]
// cargo fmt
// fn main() {
//     // let x: i32 = 132;
//     let mut x: i32 = 45;
//     x += 2;

//      println!("The value of x is: {}", x);
// }

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
    let s  = &num[..];
    println!("all elements: {:?} ", s);
}


fn string(){
    // String = vector of u8 (Vec<u8>) valid UTF-8
    // &str = slice of u8 (str) valid UTF-8

    // when to use String vs &str
    // String -> mutate or data needs to be owned ie immutable
    // &str -> data does not need to be owned ie read only

    // String
    let msg: String = String::from("Hello Rust");
    let len: usize = msg.len();
    println!()

}


#![allow(unused)]

fn main(){
    // loop
    // 1. 🔄 loop: Infinite Loop
// Runs forever unless you use break to exit it.
    loop {
        println!("loop")
    }

    let mut count = 0;
    loop {
        println!("count = {}", count);
        count += 1;
        if count == 3 {
            break; // Exit the loop
        }
    }




    // 2. 🧪 while: Conditional Loop
// Runs while a condition is true.
        let mut i = 0;
        while i < 3 {
            println!("while loop: {i}");
            i += 1;
        }
   
    



    // 3. 🔁 for: Loop Over a Range or Iterator
// Most commonly used loop in Rust.

// Automatically handles loop variable and boundaries.
    
        for i in 0..3 {  // exclusive range (0 to 2)
            println!("for loop: {i}");
        }
    
        for i in 0..=3 { // inclusive range (0 to 3)
            println!("for loop inclusive: {i}");
        }
    
        let nums = [10, 20, 30];
        for num in nums {
            println!("value: {}", num);
        }


        // 🎯 break and continue
// break — exits the loop early.

// continue — skips the current iteration and goes to the next.
    
for i in 0..5 {
    if i == 2 {
        continue; // skip 2
    }
    if i == 4 {
        break; // stop at 4
    }
    println!("i = {}", i);
}


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

    // iter 
    // aloows multiple loops 
    let v = vec![10, 20, 30, 40, 50];
    for i in v.iter {
        println!("for loop vector {i}");
    }

    
    let v = vec![10, 20, 30, 40, 50];
    for i in v.iter {
        println!("for loop vector {i}");
    }
    
}
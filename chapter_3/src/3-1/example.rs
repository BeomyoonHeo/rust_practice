//immutable example
fn immutable_example() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; //Error! this is Immutable!
    println!("The value of x is: {}", x);
}

//mutable example
fn mutable_example() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

//const exmaple
const MAX_POINTS: u32 = 100_000;

//shadowing exmple
fn shadowing_exmaple() {
    let x = 5; // bind
    let x = x + 1; // x is 6 shadow
    let x = x * 2; // x is 12 shadow
    println!("The value of x is: {}", x); // value is 12

    let mut spaces = "   "; // Warning! Declare without 'mut' keyword
    spaces = spaceses; // Error! use before 'let' keyword
}

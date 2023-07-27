// type difine example
fn type_definition_exmple() {
    let guess = "42".parse().expect("Not a number!"); //Error! not difined type
    let guess: u32 = "42".parse().expect("Not a number!"); //Success!
}

//integer type example
fn integer_example() {
    let i_8: i8 = 0; // i8 -> 8bit -> 1byte
    let i_16: i16 = 0;
    let i_32: i32 = 0;
    let i_64: i64 = 0;
    // ----------- Signed Type ------------
    let u_8: u8 = 0;
    let u_16: u16 = 0;
    let u_32: u32 = 0;
    let u_64: u64 = 0;
    //------------ Unsigned Type -------------
}


//integer literals type example
fn integer_literals_example() {
    let decimal: Decimal = 98_2222;
    let hex: Hex = 0xff;
    let octal: Octal = 0o77;
    let binary: Binary = 0b111_0000;
    let bite: Bite =b'A'; // u8 only
}

//mathematical operation example
fn mathematical_operation_example(){
    let sum = 5 + 10; //addition
    let difference = 95.5 - 4.3 //subtraction
    let product = 4 * 30 //multiplication
    let quotient = 56.7 / 33.2; //division
    let remainder = 43 % 5 //remainder

}

//boolean type example
fn boolean_type_example(){
    let t = true;
    let f: bool = false;// with explicit type annotation
}

//character type example
fn character_type_example(){
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}


//Tuple example
fn tuple_type_example(){
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value y is {}", y);
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
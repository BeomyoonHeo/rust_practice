//Snake case example
fn snake_case_example(){
    println!("This is Snake Case Example");
}

fn use_snake_case_example(){
    snake_case_example(); // this_is_snake_case_example
}

// Function argument exmple
fn function_argument_example(x: i32){
    println!("The value x is {}", x);
}

fn use_function_argument_example(){
    function_argument_example(5);
}

//Construction & Expression example
fn construction_example(){
    let x = 5; // <- this is construction because this value is not return
}

fn error_construction_example(){
    let x = (let y = 5); //Error! 
}
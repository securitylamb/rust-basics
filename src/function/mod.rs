pub fn fun() {
    println!("inside function of page function");

    another_fn();
    //  we can also pass parameter to the function
    another_fn_with_parameter(5);
    another_fn_with_parameter_2(5,'A');

    // testing expression and statement
    testing_expression_statement();

    //  function returning value in rust it is represented with ->
    let  returned_value = function_return_type();

    println!("{returned_value}");

    let  returned_value2 = function_return_type_with_parameter(5);

    println!("{returned_value2}");
}


fn another_fn(){
    println!("another function");
}


fn another_fn_with_parameter(x:i32){
    println!("the value of passed parameter is : {x}");
}

fn another_fn_with_parameter_2(x:i32,y:char){
    println!("the value of passed parameter is : {x} and {y}");
}

// expression  and statement 

/*
 * expression are  supposed to return value 
 * statement are not supposed to return value 
 * 
 * eg-> let x = 16; this is statement  means this is not supposed to return value
 * 
 * IF WE DID let x = y = 6;
 * this will give error as in rust it whuold expect a value to return;
 * as x has noting to bind to as in other langauge like C where it get value of assignment
 * so x will get value of y
 */

 fn testing_expression_statement(){
    println!("testing express and statment");
    // let x = 3;
    //  let x = x+1;
    //  this is a expression 
     let y  = {
        let x = 3;
        x+1
        // So, in short, in Rust, the last expression in a block implicitly becomes the value of that block, and no semicolon is needed after it.



        // and if i add ; at the above statement it will become statment which will give error as block should return some value as it is a expression
     };
    println!("value of x is {y}");
 }


 fn function_return_type()-> i32{
    5
 }

 fn function_return_type_with_parameter(x:i32) -> i32{
    x+1
 }
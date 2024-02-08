pub fn refrence_borrowing(){
    println!("inside refrence and borrowing ");
    /*
    It seems like you're referring to Rust programming language concepts, particularly related to ownership and borrowing. In Rust, when you pass a value to a function, by default, ownership of that value is transferred to the function. This is known as "moving" the value.

    However, if you want to allow a function to use a value without taking ownership of it, you can pass a reference to the value instead. References allow you to borrow the value temporarily, without transferring ownership. This is known as "borrowing" in Rust.
  
   */


   /*
   BASIC EXAMPLE FOR BORROWING 
    */ 
   let s1 = String::from("hello");

    // Pass a reference to the String to the function
    let len = calculate_length(&s1);

    // Since we passed a reference, s1 is still valid and usable here
    println!("The length of '{}' is {}.", s1, len);


    // NOTE -> also remember we cant update the value of stirng we have refrence to like below
    let mut s = String::from("hello");



    //  WHEN WE USE REFRENCE WE ARE USING BORROWED VALUE AND WE HAVE TO EITHER RETURN AS WE DONT OWN IT 
    change(&mut s);

    /*
        Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
     */


    // this beahaviour of not accessing mutiple mutabole refrence is that it can prevent data races at compile time which can casue any if these problem 

    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data.




    //  we can use curly braces to create new scope allowing multiple mutable refrence 
    //  check the below example
    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;

    /*
        
        We also cannot have a mutable reference while we have an immutable one to the same value.
            let mut s = String::from("hello");
            let r1 = &s; // no problem
            let r2 = &s; // no problem
            let r3 = &mut s; // BIG PROBLEM
            println!("{}, {}, and {}", r1, r2, r3);
        
     */

    //  what we can do apart from the above problem is this we can change the refrence scope 
    let s  = String::from("THIS IS A STRING");
    let r1  = &s;
    let r2= & s;
    {
        let inner_value = r2;
        println!("{inner_value}")
    }
 
   
    println!("{r1} || {r2} || {s}");


    // DANGLING POINTER

    /*
    In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
    
     */

    // example of dangling refrence 

    let refrence_to_nothing = dangle();
    println!("{refrence_to_nothing}");
    
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

// HERE WE ARE BASICALLY PASSING ADDRESS OF THE STIRNG AND IT IS A STRING SO WE DID &MUT STRING 
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// the same below danglw function will not work whenm the below function gets finished s goes out of scope  and we returning its address which will cause dangling pointing error instead we we directly returning value 
// fn dangle() -> &String{
//     let s = String::from("GAME OVER");
//     &s
// }
fn dangle() -> String{
    let s = String::from("GAME OVER");
    s
}



/*
NOTE =>
    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.

*/
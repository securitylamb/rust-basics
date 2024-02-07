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

    change(&mut s);

    /*
        Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
     */

}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
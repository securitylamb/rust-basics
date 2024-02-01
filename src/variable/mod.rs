use std::io;

pub fn variable() {
//   shadowing in rust 
    let x = 5;
    let x=x+1;
    {
        let x = x*2;
        println!("the value of inner x is :{x}")
    }
    println!("the value of outer x is :{x}");
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("the value of space is :  {spaces}");



    // the  tuple type

    let tup:(i32,f64,u8) = (500,6.4,1);
    let (x,y,z) = tup;
    // this above is destgrcuture of tuple 
    //  tuple is used to store multiple values of differemt type together

    println!("{x}");
    println!("{y}");
    println!("{z}");
    //  we can also directly access the value of the tuple directly using .
    let first: i32 = tup.0;
    let second :f64 = tup.1;
    let last :u8 = tup.2;
    println!("{first}");
    println!("{second}");
    println!("{last}");


    // ARRAY

    //  here i am telling that the arrayu will have i32 data type for variable and there will be total of 5 element 
    let a:[i32;5] = [1,2,3,4,5];
   

    //  to declare with fixed same value to all element 
    // let b = [3,5];

    let first = a[0];
    let second = a[1];
    println!("{first}");
    println!("{second}");


    // rust char literal is represented using ' ' and it is 4 byte in size and it represent unicode scaler value which helps it to store more data than just ascii Values .



    // let a:[3,5] -> it represent 3 5 times in a array


    let arr = [1,2,3,4,5];

    println!("pls enter array index");

    let mut index = String::new();
    //  creating new string literal 

    io::stdin().read_line(&mut index).expect("Failed to read line !");

    let index:usize = index.trim().parse().expect("Index entered not present");
    // it can be that rust allow usize for index

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");

    //  here in case of invalid index it will not access invalid memory but in rust it will straight way goes to exit




}

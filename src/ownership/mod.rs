pub fn ownership(){
    println!("ownership notes!!!!!!");
    /*
        this basically help us to make memory safety without the need fo garbage collection

       -> ownership is setup of rules that governs how rust manages its memory -> ion rust memory is managed by system of ownership with set of rules that compilers check if any of rules is voildated the program will not compile and non of the feature of ownership will slow down your program    




       Programming Languages and Stack/Heap Awareness:
        Stack and Heap Importance:
        Many programming languages don't emphasize stack and heap considerations.
        In systems programming languages like Rust, understanding their distinction is crucial for language behavior and decision-making.
        Stack Characteristics:
        Structure:

        LIFO (Last In, First Out) structure.
        Analogy: Stack of plates - add on top, remove from the top.
        Operations:

        Pushing onto the stack (adding data).
        Popping off the stack (removing data).
        Size Constraint:

        All stack-stored data must have a known, fixed size.
        Heap Characteristics:
        Structure:

        Less organized compared to the stack.
        Allocation Process:

        Allocating on the heap involves requesting a specific space.
        Memory allocator finds a suitable space, marks it in use, and returns a pointer.
        Pointer Usage:

        Pointer to the heap has a known, fixed size.
        Actual data retrieval requires following the pointer.
        Analogy:

        Similar to being seated at a restaurant - request space, allocator finds a spot, and returns a pointer for latecomers to locate.
        Performance Comparison:
        Speed Considerations:

        Pushing to the stack is faster than heap allocation.
        Stack location is always at the top.
        Heap allocation involves searching for space, requiring more effort.
        Access Speed:

        Accessing heap data is slower due to pointer traversal.
        Processors are more efficient when working on data close together (as on the stack).
        Function Calls and Memory Management:
        Function Execution:

        Values passed into functions, including heap pointers, and local variables, are pushed onto the stack.
        After function execution, these values are popped off the stack.
        Ownership's Role:

        Ownership addresses problems like tracking heap data usage, minimizing duplicates, and cleaning up unused data.
        Understanding ownership eliminates frequent consideration of stack and heap, emphasizing its role in managing heap data.



    Ownership Rules
    First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

    * Each value in Rust has an owner.
    * There can only be one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
    
    
    
    Variable Scope     

            {                      // s is not valid here, it’s not yet declared
                let s = "hello";   // s is valid from this point forward
                // do stuff with s
             }                      // this scope is now over, and s is no longer valid



             String example 
    This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:

             let s = String::from("hello");


     */

    //  let mut str = String::new();
    let mut s = String::from("hello");
    s.push_str(",world!!");
    println!("{}",s);


    /*
             ### Notes on String Literal vs. String Type:

#### String Literal:
- **Compile-Time Knowledge:**
  - Contents known at compile time.
  - Hardcoded directly into the final executable.
- **Efficiency and Speed:**
  - Fast and efficient due to direct inclusion.
  - Properties derived from immutability.

#### String Type (String Object):
- **Dynamic, Growable Text:**
  - Needed for mutable, growable text.
  - Requires heap allocation due to unknown size at compile time.
- **Memory Allocation:**
  - Memory requested from the heap at runtime using `String::from`.
  - Allocated amount determined during program execution.

#### Memory Management Challenges:
- **Allocation Process:**
  - Requesting memory from the allocator is a universal practice.
- **Deallocation Process:**
  - Different in languages without a garbage collector (GC).
  - Responsibility to return memory to the allocator when done with `String`.
  - Involves explicit identification of unused memory.
  - Requires manual memory deallocation.

#### Garbage Collection vs. Manual Deallocation:
- **Garbage Collector (GC):**
  - In languages with GC, it automatically tracks and cleans up unused memory.
  - Programmers don't need to explicitly manage memory deallocation.

- **Manual Deallocation:**
  - In languages without GC, programmers must identify and free unused memory.
  - Challenging: Forgetting may lead to memory waste, premature deallocation causes invalid variables, and double deallocation is a bug.

#### Responsibility and Challenges:
- **Programming Difficulty:**
  - Manual memory management has historically been a challenging problem.
  - Balancing allocation and deallocation is crucial.

- **Error-Prone Nature:**
  - Forgetting deallocation or deallocating incorrectly leads to issues.
  - Requires precision to pair one allocate with exactly one free.

#### Conclusion:
- **String Type Dynamics:**
  - `String` type facilitates dynamic, mutable text.
  - Heap allocation allows flexibility but introduces manual memory management challenges.
- **Memory Responsibility:**
  - Understanding memory deallocation is crucial for efficient resource utilization.
  - In contrast to string literals, where memory management is implicit, `String` type demands explicit attention to allocation and deallocation.
    
     */


/*
for String type ie not a string literal  
This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time
*/


/*
             DRAWBACK OF GARBAGE COLLECTION 
             Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
*/
    

    // whenever a variable goes out of scope rust calls a special function DROP .
//     let mut example_string = String::new(); // Initial length and capacity are 0.

// example_string.push_str("Hellojkgd"); // Length increases to 5, capacity may be more.

// let current_length = example_string.len(); // Retrieves the current length.
// let current_capacity = example_string.capacity(); // Retrieves the current capacity.


// println!("{current_length} and {current_capacity}");

    println!("***********************************");
    let mut example_string = String::new();
    println!("Length: {}, Capacity: {}", example_string.len(), example_string.capacity());

    example_string.push_str(" ");
    println!("Length: {}, Capacity: {}", example_string.len(), example_string.capacity());
    example_string.push_str(",  World!!!");
    println!("Length: {}, Capacity: {}", example_string.len(), example_string.capacity());
  
    /*
                 let s1 = String::from("hello");
                 let s2 = s1;
                  what if they both go out of scope this will make them memory which is pointing to the same location or we can say double free memory 
                  as the pointer of string point to same location of heap for both the string it is one of the bug of freeing memory twice 
     */

     let s1 = String::from("hello");
     let s2 = s1;
     
     println!("{}, world!", s2);
      
             /*
             in the above code if we tried 
             println!("{}, world!", s1);
             it will give bcz s1 is out scope
              */
          // if we dont want to make deep copy of string we can make it we can use clone
                        
          let s3 = String::from("hello");
          let s4 = s3.clone();
          println!("{s3} {s4} are the clone data");
          //  it actually makes a seprate copy of our string data in heap 


            //   lets talk about integet a bit 

            let x =5;
            let y =x;
            println!("x = {} and y = {}",x,y);
          // here we didnot call clone() method and still we were able to call x 
          /*
          The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.
           */

            //  COPY TRAIT 
            /*
            
            -> an anotation used in rust to know it is stored in stack 
            -> they are copied rather than moved when assigned to new variable 
            -> this allow them to remain valid even after assigment
             */

            //  IN THIS EXAMPLE I AM TESTING COPY TRAIT FOR A STRING WHICH WAS ALREADY DEFINED 

            // let new_string_1 = "mystring";
            // let new_string_2 = new_string_1;
            // println!("{new_string_1}");


            let new_string = String::from("hello");
            let new_string_2 = new_string;
            // here the new_string is called the moved value
            println!("{new_string_2}");



            //  values that can implement copy trait 

            /*
            
            All the integer types, such as u32.
            The Boolean type, bool, with values true and false.
            All the floating-point types, such as f64.
            The character type, char.
            Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
            
             */


              /*OWNERSHIP AND FUNCTION */
              let s = String ::from("hello");
              takes_ownership(s);

             let xy = 55;
             makes_copy(xy);
             /*
                println!("{s}")
                if i use this above statment this will result in the following error 
                value borrowed here after move
              */
             println!("{xy}");

                /*OWNERSHIP AND RETURN FROM FUNCTION */


                let s1 = gives_ownership();         // gives_ownership moves its return
                // value into s1

                let s2 = String::from("hello");     // s2 comes into scope
              //   s2 will go outof scope after this line 
                let s3 = takes_and_gives_back(s2);  
             

                println!("{s1}");
                // here  for s2 we have passed the value to the function present in s3 
                //  so wencant use s2 after that 
                println!("{s3}");


                /*

                NOTE -> if we want to use the value which is send to function but we want function not to take ownership  we have to passback the value that we passed

                */
                let s1 = String::from("helloddd");
                let (s2, len) = calculate_length(s1);

                println!("The length of '{}' is {}.", s2, len);

                
    }
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.


    fn gives_ownership() -> String {             // gives_ownership will move its
      // return value into the function
      // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
          // moves out to the calling
          // function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                  // scope

    a_string  // a_string is returned and moves out to the calling function
    }
    fn calculate_length(s: String) -> (String, usize) {


      let length = s.len(); // len() returns the length of a String
  
      (s, length)
  }
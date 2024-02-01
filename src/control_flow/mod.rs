pub fn flow(){
    println!("GO WITH THE FLOW");
    let num = 4;
    if num <5{
        println!("less than 5")
    }else{
        println!("greator than 5")
    }


    //  keep in mind the  value passed in if condition should be boolean otherwise it will give error like the below code 

    // if num {
    //     println!(" number 4 was there ")
    // }
    

    // if/else condition when value is being passed to let expression 
    
    //  this one works fine 
    let condition = true;
    let result = if condition {5} else{6};
    println!("{result}");

    // this one gives error bcz at compile time there is 2 possible value of result one Integer and other one is string 
    // let result = if condition {5} else{ "string"};
    // println!("{result}")

        // LOOP IN RUST AND ITS USAGE

        let mut counter = 0;
        let result_loop = loop{
            counter+=1;
            if counter == 10{

                //  counter*2 bcz i need to pass this value for my let result_loop
                break counter*2;
            }
        };
        println!("{result_loop}");



}
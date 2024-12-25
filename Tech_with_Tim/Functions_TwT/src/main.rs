fn main() {
    //Functions
    //Naming Convention in Rust is to use snake case (function_name)
    //Function declaration syntax
    fn test_one(){
        println!("Test is called!");
    }

    test_one();

    //Functions with parameters
    fn add_numbers(x:i32 , y:i32){   //We need to explicitly define the type of parameters and multiple parameters are seperated by a comma
        println!("The sum is : {}",x+y) //It will not return if we put a semicolon(;) here , Expressions vs statements
    }

    add_numbers(20,30);

}



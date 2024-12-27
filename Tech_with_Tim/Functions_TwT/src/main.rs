fn main() {
    //Functions
    //Naming Convention in Rust is to use snake case (function_name)
    //Function declaration syntax
    fn test_one(){
        println!("Test is called!");
    }

    test_one();
    add_numbers(20,30);
    add_numbers(5,7);

    //Expressions vs Statements
    println!("Hello World!"); //Statement
    add_numbers(50,80); //function call is also a statement


    let number = {  //This is a statement
        let x = 3;  //but
        x+1         //This is expression  
    };

    
    let result = add_numbers2(10,20);
    println!("{}",result);

    let result2 = add_numbers3(result,40);
    println!("{}",result2);





}

//Functions with parameters
fn add_numbers(x:i32 , y:i32){   //We need to explicitly define the type of parameters and multiple parameters are seperated by a comma
    println!("The sum is : {}",x+y) //It will not return if we put a semicolon(;) here , Expressions vs statements
}

//Returning from function
fn add_numbers2(x:i32 , y :i32) -> i32 { // -> is return operator followed by the type of return data
    x+y //Expression (Statement without semicolon)
}

//Explicit Return Keyword
fn add_numbers3(x:i32 , y:i32) -> i32{
    return x+y; //semicolon (;) is allowed with return keyword in statements
}

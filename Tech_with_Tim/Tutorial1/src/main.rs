fn main() {
    //Hello World 
    println!("Hello World");

    //Another Way
    print!("Hello World");

    //Variables
    let a = "Hitman";

    //Printing a variable using default formatter
    print!("{}",a);

    //Explicit Type Defination of a variable
    let num : i32 = 24;
    println!("{}",num);

    //implicit/Default Type Defination
    let num2 = -37;
    print!("{}",num2);

    //Mutable Variables
    let mut num3 = 1;
    println!("Num 3 is :{}",num3);
    num3 = num3+2;
    println!("Num 3 is :{}",num3);

    //Redefining 
    println!("Num 2 is :{}",num2);
    let num2 = 37;
    println!("Redifined Num 2 is :{}",num2);

    //Constants
    const SECONDS_IN_MINUTE :i8 = 60 ;
    println!("1 minute = {}",SECONDS_IN_MINUTE);

    //Mutating or Redifining the constants will throw an error 

    //Name Shadowing 
    println!("Num 3 is :{}",num3);
    {
        //Other Scope
        let num3 = num3-2;
        println!("Num 3 is :{}",num3);
    }





}

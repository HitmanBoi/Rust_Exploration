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

}

fn main() {
    //Strings

    //String Literal
    let greeting:&str = "Hello World"; //not changable at runtime
    println!("{}",greeting);

    //Dynamic String
    let greeting:String = String :: from("hello world");
    println!("{}",greeting)


    //Advance (Accessing a part of string)

    //println!("{}",greeting[0]);   It is a wrong method in rust
     
}

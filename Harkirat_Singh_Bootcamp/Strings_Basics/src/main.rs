fn main() {
    //Strings

    //String Literal
    let greeting:&str = "Hello World"; //not changable at runtime
    println!("{}",greeting);

    //Dynamic String
    let greeting:String = String :: from("hello world");
    println!("{}",greeting)
     
}

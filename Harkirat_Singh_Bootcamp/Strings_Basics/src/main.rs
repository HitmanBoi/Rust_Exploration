fn main() {
    //Strings

    //String Literal
    let greeting:&str = "Hello World"; //not changable at runtime
    println!("{}",greeting);

    //Dynamic String
    let greeting:String = String :: from("hello world");
    println!("{}",greeting);


    //Advance (Accessing a part of string)

    //println!("{}",greeting[0]);   It is a wrong method in rust

    let char1 : Option<char> = greeting.chars().nth(0); //Option<chars> .. Could be char , could be nothing
    //println!("{}",char1); //Unsafe access to memory

    println!("{}",char1.unwrap()); //unwrap .. returns the result only if result is present , Allowing the okay with the runtime exception
    
    //Another Method is Pattern Matching (yet to learn)
    
     
}

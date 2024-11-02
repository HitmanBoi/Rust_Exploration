fn main() {
    let mut x = 5;
    println!("x is: {}", x);

    {
        x = "Hello";
        println!("x is: {}", x);
    }

    x = x+1;
    println!("x is: {}", x);
}

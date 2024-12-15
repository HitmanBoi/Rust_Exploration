fn main() {
    //Type Casting

    let x = 255.0;
    let y = 10.0;
    //default type = f64

    //Type casting 
    //numero uno 
    let x = 255.0f32 ;
    //overflow error
    //let of = 255i8 ;

    //method 1 readability
    let x = 127_i8 ;

    //readability elaborated

    let x = 127_000_i64 ;

    //method 2
    //using "as" keyword

    let x = 127_000 as i64;  //explicit type conversion

    //runtime implementation

    let x = 127_000 as i64 ;
    let y = 10_i32;
    let z = 127_000 / (y as i64);
    println!("{}",z);



}

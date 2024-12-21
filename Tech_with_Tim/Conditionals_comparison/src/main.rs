fn main() {
    //Conditionals 
    //Condition is just any expresssion that evaluates to true or false or the Boolean data type

    // < less than
    let cond = 1 < 2 ;
    println!("1 < 2 is {}",cond);

    // > greater than
    let cond = 1 > 2;
    println!("1 > 2 is {}",cond);

    // <= less than equals to
    let cond = 2 >= 2;
    println!("2 >= 2 is {}",cond);

    // >= greater than equals to 
    let cond = 5 <= 2;
    println!("5 <= 2 is {}",cond);

    // != not equals to 
    let cond = 5 != 4;
    println!("5 != 4 is {}",cond);

    // == is equals to
    let cond = 5 == 5;
    println!("5 == 5 is {}",cond);

    //Note : can only compare same data types

    //Compound Conditional
    //Multiple conditions combined together using logical operators 

    // && ( and operator )
    let cond2 = true && cond ;
    println!("true && {} is {}",cond , cond2);

    // || ( or operator )
    let cond2 = false || cond ;
    println!("false || {} is {}",cond , cond2);



}

fn main() {
    //Primitibve Data Types

    //Numbers 

    //Signed Intergers (i8 i16 i32 i64 i128)

    let num1:i8 = i8::MIN ; //-128 to 127
    println!("The i8 type of signed integers ranges from {} to {}.",num1,i8::MAX);

    let num2:i16 = i16::MAX; //-32768 to 32767
    println!("The i16 type of signed integers ranges from {} to {}.",i16::MIN,num2);

    let mut num3 = 24 ; //-2147483648 to 2147483647 , i32 is default implicit type for integers
    print!("The i32 type of signed intergers ranges from {}",i32::MIN);
    num3 = i32::MAX;
    println!(" to {}.",num3);

    let _num4:i64 = 24 ; //-9223372036854775808 to 9223372036854775807
    println!("The i64 type of signed integers ranges from {} to {}.",i64::MIN,i64::MAX);

    let num5:i128 = 170141183460469231731687303715884105727 ; //-170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
    println!("The i128 type of signed integers ranges from {} to {}.",i128::MIN,num5);

    //Trying Something
    //println!("{}",i28::MAX);

    //Unsigned Integers (u8 u16 u32 u64 u128)
    let num6 : u8 = u8::MAX; // 0 to 255
    println!("The u8 type of unsigned integers ranges from 0 to {}.",num6);

    let num7 : u16 = u16::MAX; // 0 to 65535
    println!("The max value an unsigned integer u16 can store is {}.",num7);

    let num8 : u32 = 429_496_729_5; //0 to 4294967295
    println!("Max value of u32 is {} , yep {}.",u32::MAX,num8);

    let mut num9 : u64 = u64::MIN; //
    print!("The u64 ranges from {}",num9);
    num9 = u64::MAX;
    println!(" to {}",num9);

    let _num10 : u128 = 1 ;
    println!("The number of positive integers an u128 data type can contain is {} and a 0",u128::MAX);


    //Floating point numbers (f32 f64)
    let flnum1 : f32 = 50000.526175865787887788686785778676568;
    println!("50000.526175865787887788686785778676568 in f32 will be represented as {}",flnum1);
    println!("f32 is also known as single precesion");

    let flnum2 = 50000.526175865787887788686785778676568;
    println!("50000.526175865787887788686785778676568 in f64 will be represented as {}",flnum2);
    println!("f64 is also known as double precesion it is default float type by implicit decesion");

    //Booleans
    let bul1 : bool = false;
    let bul2 : bool = true;
    println!("0 is represented as {} and 1 is represented as {} in boolean values",bul1,bul2);

    //Strings
    let mut str1 : &str = "Hitman";
    println!("{}",str1);
    









    














}

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
    let num6 : u8 = u8::MAX;
    println!("The u8 type of unsigned integers ranges from 0 to {}",num6);










}

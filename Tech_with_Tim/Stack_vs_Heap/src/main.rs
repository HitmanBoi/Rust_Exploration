fn main() {
    //Stack and Heap

    //Stack
    //The data having fixed size in memory is stored inside stack in RAM and it follows general (LIFO / FILO) operation

    //example
    let x = 2;
    let y = x;
    println!("y i.e. {} is pooped",y);
    println!("x i.e. {} is pooped",x);

    //Heap
    //The data having/requiring dynamic size in memory is stored in heap in RAM and this heap is different from that data type.
    //The adderess of data as in heap is stored in the stack as pointer referance.

    //example
    let x = 2;
    let String1 = String::from("Hello"); //Dynamic String

    println!("String1 is popped from stack");
    println!("{} is removed from Heap since owner (i.e. String1) is popped",String1);
    println!("x i.e. {} is popped from stack",x);

}

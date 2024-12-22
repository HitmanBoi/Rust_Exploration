fn main() {
    //Control Flow 
    //if..else
    let food = "bread";
    if food == "bread"{
        println!("That sounds boring");
    }
    else{
        println!("Oh! that's not good");
    }

    //else if
    let food = "cookie";
    if food == "bread"{
        println!("That sounds boring");
    }
    else if food == "cookie"{
        println!("I love cookies too");
    }
    else{
        println!("Oh! that's not good");
    }

    //nested if
    let food = "bread";
    let taste = "sweet";
    if food == "bread"{
        println!("{} sounds boring",food);
        if taste == "sweet"{
            println!("Whatever , I don't like {}",food);
        }
    }
    else {
        println!("{} , Oh! that's not good",food);
    }



}

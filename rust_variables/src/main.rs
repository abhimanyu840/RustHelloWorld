fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 6;
    // println!("The new value of x is: {}", x);

    // let x = 6;
    // let x = x + 1;
    // println!("The new value of x is: {}", x);

    // let x = "Hello World";
    // println!("Now the x is: {}", x);

    // x = "Hello World"; //Not allowed
    {
        let x = 9;
        println!("The value of x is: {}", x);
    }

    let x = x + 1;
    println!("The value of x is: {}", x);

    //Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum point is: {}", MAX_POINTS);
}

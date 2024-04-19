// fn another_function(x: i32, y: char) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// fn main() {
//     println!("Functions in Rust");
//     another_function(5, 'a');
// }

//Return values form functions

// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }
fn sum_diff(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

fn main() {
    // //Statements and Expressions
    // let x = 5; //Statement

    // let y = {
    //     let x = 3;
    //     x + 1 //Expression
    // };

    // println!("The value of y is: {}", y);
    // let (x, y) = sum_diff(22, 44);
    // println!("The sum is {}, and the diff is {}", x, y);
    let x = sum_diff(22, 44);
    println!("The sum is {}, and the diff is {}", x.0, x.1);
    println!("The sum and diff is {:?}", x)
}

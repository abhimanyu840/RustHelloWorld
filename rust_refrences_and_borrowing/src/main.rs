// fn main() {
//     let s = String::from("Hello");
//     let len = calculate_length(&s);
//     println!("The size of `{}` is {}", s, len);
// }

// //Function to calculate length

// fn calculate_length(s: &String) -> usize {
//     let len = s.len();
//     len
// }

// fn main() {
//     //Mutable References
//     let mut s = String::from("Hello");
//     change(&mut s);

//     println!("{}", s);
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     //Multiple mutable references
//     let mut s = String::from("Hello");
//     {
//         let r1 = &mut s;
//         r1.push_str(" World");

//     }
//     let r2 = &mut s;
//     println!("{}", r2);
// }

//Mutable and Immutable References
// fn main() {
//     let mut s = String::from("Hello");
//     let r1 = &s;
//     let r2 = &s;

//     println!("{} and {}", r1, r2);

//     let r3 = &mut s;
//     println!("{}", r3);
// }

// // Dangling References
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
fn main() {
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

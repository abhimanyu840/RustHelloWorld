#[derive(Debug)]
// struct User {
//     name: String,
//     email: String,
//     is_active: bool,
//     age: u8,
// }

// fn main() {
//     //Instance of struct
//     let mut user1 = User {
//         name: String::from("Abhimanyu Kumar"),
//         email: String::from("abc@gmail.com"),
//         is_active: true,
//         age: 20,
//     };
//     //Print all the value in structs
//     println!(
//         "Name: {} \n Email: {} \n Is Active: {} \n Age: {}",
//         user1.name, user1.email, user1.is_active, user1.age
//     );
//     //Update the value of struct
//     user1.name = String::from("Abhimanyu Kumar Singh");
//     println!(
//         "Name: {} \n Email: {} \n Is Active: {} \n Age: {}",
//         user1.name, user1.email, user1.is_active, user1.age
//     );
// }

// fn main() {
//     // Create a user
//     let user1 = build_user(
//         String::from("Abhimanyu Kumar"),
//         String::from("abc@gmail.com"),
//     );

//     //Print all the values
//     println!(
//         "Name: {} \nEmail: {} \nis_active: {} \nage: {}",
//         user1.name, user1.email, user1.is_active, user1.age
//     );
// }

// //create a function to build a user
// // fn build_user(name: String, email: String, is_active: bool, age: u8) -> User {
// //     User {
// //         name,
// //         email,
// //         is_active,
// //         age,
// //     }
// // }
// fn build_user(name: String, email: String) -> User {
//     User {
//         name,
//         email,
//         is_active: true,
//         age: 20,
//     }
// }

// fn main() {
//     //Creating an instance of struct
//     let user1 = User {
//         name: String::from("Abhimanyu Kumar"),
//         email: String::from("abhi@abhimail.com"),
//         is_active: true,
//         age: 20,
//     };

//     let user2 = User {
//         name: String::from("Abhimanyu Kumar Singh"),
//         email: user1.email.clone(),
//         is_active: user1.is_active,
//         age: user1.age,
//     };

//     println!(
//         "Name: {} \nEmail: {} \nis_active: {} \nage: {}",
//         user2.name, user2.email, user2.is_active, user2.age
//     );

//     println!(
//         "Name: {} \nEmail: {} \nis_active: {} \nage: {}",
//         user1.name, user1.email, user1.is_active, user1.age
//     );
// }

//Tuple Structs
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("Black: {} {} {}", black.0, black.1, black.2);
//     println!("Origin: {} {} {}", origin.0, origin.1, origin.2);
// }

//Unit-like Struct
struct User;

fn main() {
    let user = User;
    println!("{:?}", user);
}

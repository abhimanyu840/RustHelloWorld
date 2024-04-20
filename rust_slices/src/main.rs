// fn main() {
//     let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

//     let slice: &[char] = &arr[1..3];

//     println!("sliced value is {:?}", slice);

//     let vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

//     let slicedvec: &[char] = &vec[1..5];

//     println!("sliced value is {:?}", slicedvec);

//     //Slicing of string
//     let str = String::from("Abhimanyu");
//     let slicedstr: &str = &str[..];

//     println!("Sliced string is {:?}", slicedstr);

//     //Slicing of Array of string
//     let str_array: [&str; 5] = ["Abhimanyu", "Kumar", "Singh", "Another", "String"];

//     let slice_str_array: &[&str] = &str_array[1..=3];

//     println!("Sliced Array of str is {:?}", slice_str_array);

//     let string_array: [String; 5] = [
//         String::from("Abhimanyu"),
//         String::from("Kumar"),
//         String::from("Singh"),
//         String::from("Another"),
//         String::from("String"),
//     ];

//     let slice_string_array: &[String] = &string_array[2..4];

//     println!("Sliced Array of string is {:?}", slice_string_array);
// }

//Getting first word of sentence(Without using Slice)

// fn main() {
//     let sentence = String::from("This is a sentence ");

//     let returned = first_word(&sentence);

//     println!("{:?}",returned)
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     println!("bytes: {:?}", bytes);

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

//Getting first word of sentence(using Slice)
fn main() {
    let sentence = String::from("This is a sentence ");

    let returned = first_word(&sentence);

    println!("{:?}", returned)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

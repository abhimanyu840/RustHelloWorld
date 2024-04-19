struct Person {
    name: String,
    age: u8,
}

enum Direction {
    Up,

    Down,

    Left,

    Right,
}

fn main() {
    println!("Data Types in Rust");
    // Scalar Types: -
    // Integers

    // let small_number: u8 = 22;
    // let big_number: u128 = 84748393999938383883822;
    // let small_number2: i8 = -22;
    // let big_number2: i128 = 84748393999938383883822;

    // println!("small_number: {}", small_number);
    // println!("big_number: {}", big_number);
    // println!("small_number2: {}", small_number2);
    // println!("big_number2: {}", big_number2);

    // let decimal = 98_222;
    // let hex = 0xff;
    // let octal = 0o77;
    // let binary = 0b1111_0000;
    // let byte = b'A';

    // println!("decimal: {}", decimal);
    // println!("hex: {}", hex);
    // println!("octal: {}", octal);
    // println!("binary: {}", binary);
    // println!("byte: {}", byte);

    // // Floating Point
    // let x = 2.0; //default f64
    // let y: f32 = 3.0;

    // println!("x: {}, y: {}", x, y);

    // //Numeric Operations
    // let sum = x + y;

    // let diff = x - y;

    // let product = x * y;

    // let quotient = x / y;

    // let remainder = 43 % 5;

    // println!("Sum: {}", sum);

    // println!("Difference: {}", diff);

    // println!("Product: {}", product);

    // println!("Quotient: {}", quotient);

    // println!("Remainder: {}", remainder);

    //Booleans
    // let t = true; //implicit Declaration

    // let f: bool = false; // Explicit Declaration

    // println!("t: {}, f: {}", t, f);

    // //if
    // if t {
    //     println!("t is true");
    // } else {
    //     println!("t is false");
    // }

    // let not_t = !t;
    // println!("not_t: {}", not_t);

    //Characters

    // let c = 'z';
    // let z:char = 'ℤ';

    // let heart_eyed_cat = '😻';

    // println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // //iterate over characters in a string
    // for char in "Hello! World 🙏 ".chars() {
    //     println!("{}", char);
    // }

    // //Tuples
    // let tup: (i32, f64, char) = (500, 3.6, 'x');

    // //Destructuring Tuples
    // let (x, y, z) = tup;
    // println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);
    // println!("The value of z is: {}", z);

    // //Accessing Tuples by Index

    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let x = tup.2;

    // println!("five_hundred: {}", five_hundred);
    // println!("six_point_four: {}", six_point_four);
    // println!("x: {}", x);

    // //Arrays
    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];
    // let second = a[1];
    // println!("first: {}", first);
    // println!("second: {}", second);

    // //Iterating array
    // for element in a.iter() {
    //     println!("Element: {}", element);
    // }

    //Custom Types
    //Structs

    let person = Person {
        name: String::from("Abhimanyu Kumar"),
        age: 20,
    };
    println!("Person: {}", person.name);
    println!("Age: {}", person.age);

    //Enums
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Player is going up"),
        Direction::Down => println!("Player is going down"),
        Direction::Left => println!("Player is going left"),
        Direction::Right => println!("Player is going right"),
    }
}

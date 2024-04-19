# Rust Data Types

There are three type of data types in Rust

## Scalar Types

A scalar type represents a single value.

-   Integers
-   Floating Point Numbers
-   Booleans
-   Characters

## Compound Types

-   Tuples
-   Arrays

## Custom Types

-   Structs
-   Enums

## Scalar Types

### 1. Integers

-   Integer Types in Rust

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

-   Integer Literals in Rust

| Number literals | Example        |
| --------------- | -------------- |
| Decimal         | 98_222         |
| Hex             | 0xff           |
| Octal           | 0o77           |
| Binary          | 0b1111_0000    |
| Byte            | (u8 only) b'A' |

### 2. Floating Point Numbers

Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

```
let x = 2.0; // f64
let y: f32 = 3.0; // f32
```

-   Numeric Operations

```
   let sum = x + y;

    let diff = x - y;

    let product = x * y;

    let quotient = x / y;

    let remainder = 43 % 5;

    println!("Sum: {}", sum);

    println!("Difference: {}", diff);

    println!("Product: {}", product);

    println!("Quotient: {}", quotient);

    println!("Remainder: {}", remainder);
```

### 3. Booleans

```
let t = true; // with implicit type annotation

let f: bool = false; // with explicit type annotation
```

> booleans have to be initialized at the time of declaration or else it will throw an error

### 4. Characters

> For character we use `char` as type for eg:-

```
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
```

> We can also iterate char like this

```
    for c in "Hello नमस्ते".chars() {
        println!("{}", c);
    }
```

## Compound Types

### 1. Tuples

> Tuples are fixed length and immutable

```
    let tup: (i32, f64, char) = (500, 3.6, 'x');
```

-   Destructuring
    We can access the values of a tuple using a pattern much like a destructuring struct.

```
    let tup = (500, 3.6, 'x');
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
```

-   Accessing tuples with index

```
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let x = tup.2;

    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("x: {}", x);
```

### 2. Arrays

> In rust arrays are a collection of values of the same type.

-   Arrays are fixed length
-   Arrays are homogeneous

```
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("first: {}", first);
    println!("second: {}", second);
```

> Iterating Array

```
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
```

## Custom Types

### 1. Structs

> We can define structs inside main function but usually we define structs at the top level i.e. above the main function.

```
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let person = Person {
        name: String::from("Abhimanyu Kumar"),
        age: 20,
    };

    println!("{} is {}", person.name, person.age);
}
```

### 2. Enums

> Enums are types that have a few definite values.

```
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Player is going up"),
        Direction::Down => println!("Player is going down"),
        Direction::Left => println!("Player is going left"),
        Direction::Right => println!("Player is going right"),
    }
}
```

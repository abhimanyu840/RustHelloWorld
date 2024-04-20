# Structs

-   Structs are used to create custom data types.

-   Structs are immutable by default.

-   Structs can be used to create complex data types.

```
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8,
}

fn main() {
    let user1 = User {
        name: String::from("Abhimanyu Kumar"),
        email: String::from("abc@gmail.com"),
        is_active: true,
        age: 20,
    };
    //Print all the value in structs
    println!(
        "Name: {} \n Email: {} \n Is Active: {} \n Age: {}",
        user1.name, user1.email, user1.is_active, user1.age
    );
}

```

> We can mutate it by adding `mut` keyword

```
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8,
}

fn main() {
    //Instance of struct
    let mut user1 = User {
        name: String::from("Abhimanyu Kumar"),
        email: String::from("abc@gmail.com"),
        is_active: true,
        age: 20,
    };
    //Print all the value in structs
    println!(
        "Name: {} \n Email: {} \n Is Active: {} \n Age: {}",
        user1.name, user1.email, user1.is_active, user1.age
    );
    //Update the value of Struct
    user1.name = String::from("Abhimanyu Kumar Singh");
    println!(
        "Name: {} \n Email: {} \n Is Active: {} \n Age: {}",
        user1.name, user1.email, user1.is_active, user1.age
    );
}
```

# Build a struct with a function

```
fn main() {
    // Create a user
    let user1 = build_user(
        String::from("Abhimanyu Kumar"),
        String::from("abc@gmail.com"),
    );

    //Print all the values
    println!(
        "Name: {} \nEmail: {} \nis_active: {} \nage: {}",
        user1.name, user1.email, user1.is_active, user1.age
    );
}

//create a function to build a user
 fn build_user(name: String, email: String, is_active: bool, age: u8) -> User {
     User {
         name,
         email,
         is_active,
         age,
     }
 }

```

> We can also define some value in function

```

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        is_active: true,
        age: 20,
    }
}

```

## Creating an instance of struct

```
fn main() {
    //Creating an instance of struct
    let user1 = User {
        name: String::from("Abhimanyu Kumar"),
        email: String::from("abhi@abhimail.com"),
        is_active: true,
        age: 20,
    };

    let user2 = User {
        name: String::from("Abhimanyu Kumar Singh"),
        email: user1.email,
        is_active: user1.is_active,
        age: user1.age,
    };

    println!(
        "Name: {} \nEmail: {} \nis_active: {} \nage: {}",
        user2.name, user2.email, user2.is_active, user2.age
    );
}
```

> But after doing this we will not be able to print the value of `user1.name` because user2 takes ownership of it. If we want using user1 we will have to clone its value in user2 by using `.clone()`.

```
    let user2 = User {
        name: String::from("Abhimanyu Kumar Singh"),
        email: user1.email.clone(),
        is_active: user1.is_active,
        age: user1.age,
    };

```

## Tuple Structs

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Unit Like Structs

-   Struct that have no fields and no values.

-   It is basically used when we don't know what will be the value of the struct but will be.

```
struct UnitLikeStruct;

fn main() {
    let unit_like_struct = UnitLikeStruct;

    println!("Unit Like Struct: {:?}", unit_like_struct);
}
```
